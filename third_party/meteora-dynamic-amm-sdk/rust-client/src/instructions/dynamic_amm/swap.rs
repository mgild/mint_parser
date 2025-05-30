use self::fee_estimation::DEFAULT_COMPUTE_UNIT;
use crate::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use anchor_spl::associated_token::get_associated_token_address;
use prog_dynamic_amm::state::Pool;
use prog_dynamic_vault::state::Vault;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
#[derive(Parser, Debug, Clone)]
pub struct SwapDynamicAmmArgs {
    #[clap(long, env)]
    pub pool: Pubkey,
    #[clap(long, env)]
    pub source_token: Pubkey,
    #[clap(long, env)]
    pub in_amount: u64,
    #[clap(long, env)]
    pub minimum_out_amount: u64,
}

pub fn process_swap_dynamic_pool(args: &Args, sub_args: &SwapDynamicAmmArgs) {
    let SwapDynamicAmmArgs {
        pool,
        in_amount,
        minimum_out_amount,
        source_token,
    } = sub_args;

    let client = RpcClient::new_with_commitment(&args.rpc_url, CommitmentConfig::finalized());
    let keypair = read_keypair_file(&args.keypair_path.clone().unwrap()).unwrap();

    let mut ixs = vec![
        ComputeBudgetInstruction::set_compute_unit_price(args.priority_fee),
        ComputeBudgetInstruction::set_compute_unit_limit(DEFAULT_COMPUTE_UNIT),
    ];

    let program_amm_client = args.to_rpc_args().get_program_client(prog_dynamic_amm::ID);
    let pool_state: Pool = program_amm_client.account(*pool).unwrap();

    let program_vault_client = args.to_rpc_args().get_program_client(prog_dynamic_amm::ID);
    let a_vault_state: Vault = program_vault_client.account(pool_state.a_vault).unwrap();
    let b_vault_state: Vault = program_vault_client.account(pool_state.b_vault).unwrap();

    let (user_destination_token, protocol_token_fee, destination_mint) =
        if *source_token == pool_state.token_a_mint {
            (
                get_associated_token_address(&keypair.pubkey(), &pool_state.token_b_mint),
                pool_state.protocol_token_a_fee,
                pool_state.token_b_mint,
            )
        } else {
            (
                get_associated_token_address(&keypair.pubkey(), &pool_state.token_a_mint),
                pool_state.protocol_token_b_fee,
                pool_state.token_a_mint,
            )
        };

    if client.get_account(&user_destination_token).is_err() {
        ixs.push(
            spl_associated_token_account::instruction::create_associated_token_account(
                &keypair.pubkey(),
                &keypair.pubkey(),
                &destination_mint,
                &spl_token::ID,
            ),
        );
    }

    ixs.push(Instruction {
        program_id: prog_dynamic_amm::ID,
        accounts: prog_dynamic_amm::accounts::Swap {
            pool: *pool,
            user_source_token: get_associated_token_address(&keypair.pubkey(), source_token),
            user_destination_token,
            a_vault_lp: pool_state.a_vault_lp,
            b_vault_lp: pool_state.b_vault_lp,
            a_vault: pool_state.a_vault,
            b_vault: pool_state.b_vault,
            a_vault_lp_mint: a_vault_state.lp_mint,
            b_vault_lp_mint: b_vault_state.lp_mint,
            a_token_vault: a_vault_state.token_vault,
            b_token_vault: b_vault_state.token_vault,
            user: keypair.pubkey(),
            vault_program: prog_dynamic_vault::ID,
            token_program: spl_token::ID,
            protocol_token_fee,
        }
        .to_account_metas(None),
        data: prog_dynamic_amm::instruction::Swap {
            in_amount: *in_amount,
            minimum_out_amount: *minimum_out_amount,
        }
        .data(),
    });

    let blockhash = client.get_latest_blockhash().unwrap();
    let tx =
        Transaction::new_signed_with_payer(&ixs, Some(&keypair.pubkey()), &[&keypair], blockhash);
    let payload = args
        .to_rpc_args()
        .send_transaction_wrapper(
            &tx,
            MAX_RETRIES,
            keypair.pubkey(),
            "".to_string(),
            sucess_cb,
            failed_cb,
        )
        .unwrap();
    let mut result = BTreeMap::new();
    result.insert(0, Some(payload));
    handle_collect_cb_by_tx_action(args.tx_action, None, &result);
}

fn sucess_cb(_wallet_memo: String, sig: Signature) {
    println!("done swap {:?}", sig);
}
fn failed_cb(_wallet_memo: String) {
    println!("cannot swap");
}
