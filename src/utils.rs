use reth::primitives::{address, Address, U256, Bytes, keccak256};
use alloy_dyn_abi::DynSolValue;

pub fn tax_checker_address() -> Address {
    address!("00000000000000000000000000000000F3370000")
}

// Holds constant value representing braindance caller
pub fn tax_checker_controller_address() -> Address {
    address!("000000000000000000000000000000000420BABE")
}

pub fn get_tax_checker_code() -> Bytes {
    "608060405234801561000f575f80fd5b5060043610610029575f3560e01c8063d7ad21ac1461002d575b5f80fd5b61004061003b366004610dca565b610059565b6040805192835260208301919091520160405180910390f35b6040805160e0810182525f818301819052606080830182905260a0830182905260c083018290526001600160a01b038781168085529087166020850152608084018690528451630240bc6b60e21b8152945192948594939192630902f1ac926004808401938290030181865afa1580156100d5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100f99190610e23565b506001600160701b0390811660c08401521660a082015280516040805163d21220a760e01b815290516001600160a01b039092169163d21220a7916004808201926020929091908290030181865afa158015610157573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061017b9190610e6f565b6001600160a01b031681602001516001600160a01b0316036101b65760c08101805160a0830180516001600160701b03908116909352911690525b80602001516001600160a01b03166323b872dd825f0151306127108560a001516101e09190610eb9565b6040516001600160e01b031960e086901b1681526001600160a01b0393841660048201529290911660248301526001600160701b031660448201526064016020604051808303815f875af115801561023a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061025e9190610ede565b50805f01516001600160a01b031663fff6cae96040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561029a575f80fd5b505af11580156102ac573d5f803e3d5ffd5b50505050805f01516001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156102ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103139190610e23565b506001600160701b0390811660c08401521660a08201526020808201516001600160a01b039081166040808501919091528351815163d21220a760e01b81529151600194919093169263d21220a79260048082019392918290030181865afa158015610381573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103a59190610e6f565b6001600160a01b031682602001516001600160a01b03160361045f5760c08201805160a0840180516001600160701b0390811690935291169052815160408051630dfe168160e01b815290516001600160a01b0390921691630dfe1681916004808201926020929091908290030181865afa158015610426573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061044a9190610e6f565b6001600160a01b03166060830152505f6104d1565b815f01516001600160a01b031663d21220a76040518163ffffffff1660e01b8152600401602060405180830381865afa15801561049e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104c29190610e6f565b6001600160a01b031660608301525b5f6104dc83836105dc565b6104e7906001610efd565b9050825f01516001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa158015610528573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061054c9190610e23565b506001600160701b0390811660c08601521660a0840152816105875760c08301805160a0850180516001600160701b03908116909352911690525b5f61059284846109df565b60408051848152602081018390529192507fbcf2857ab072dd1bb2474056a1d6cd22f44ddef1f02199e5003cef746a37be34910160405180910390a1909890975095505050505050565b60208201516040516370a0823160e01b81523060048201525f9182916001600160a01b03909116906370a0823190602401602060405180830381865afa158015610628573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061064c9190610f16565b90505f610679828660a001516001600160701b03168760c001516001600160701b03168860800151610d65565b6040868101518751915163a9059cbb60e01b81526001600160a01b03928316600482015260248101869052929350839291169063a9059cbb906044016020604051808303815f875af11580156106d1573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106f59190610ede565b5060a0860151604080880151885191516370a0823160e01b81526001600160a01b03928316600482015286936001600160701b031692909116906370a0823190602401602060405180830381865afa158015610753573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107779190610f16565b6107819190610f2d565b1461083c5760a0860151604080880151885191516370a0823160e01b81526001600160a01b039283166004820152610839936001600160701b031692909116906370a0823190602401602060405180830381865afa1580156107e5573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108099190610f16565b6108139190610f2d565b8760a001516001600160701b03168860c001516001600160701b03168960800151610d65565b91505b6108486101f483610f2d565b915084156108c2578551604080516020810182525f808252915163022c0d9f60e01b81526001600160a01b039093169263022c0d9f9261089092909187913091600401610f83565b5f604051808303815f87803b1580156108a7575f80fd5b505af11580156108b9573d5f803e3d5ffd5b5050505061092f565b8551604080516020810182525f808252915163022c0d9f60e01b81526001600160a01b039093169263022c0d9f92610901928792309190600401610f83565b5f604051808303815f87803b158015610918575f80fd5b505af115801561092a573d5f803e3d5ffd5b505050505b60608601516040516370a0823160e01b81523060048201525f916001600160a01b0316906370a08231906024015b602060405180830381865afa158015610978573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061099c9190610f16565b6109a69083610f2d565b9050805f036109b7575f94506109d5565b816109c482612710610faf565b6109ce9190610fc6565b61ffff1694505b5050505092915050565b60608201516040516370a0823160e01b81523060048201525f9182916001600160a01b03909116906370a0823190602401602060405180830381865afa158015610a2b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a4f9190610f16565b90505f610a7c828660c001516001600160701b03168760a001516001600160701b03168860800151610d65565b6060860151865160405163a9059cbb60e01b81526001600160a01b03918216600482015260248101869052929350839291169063a9059cbb906044016020604051808303815f875af1158015610ad4573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610af89190610ede565b5060c0860151606087015187516040516370a0823160e01b81526001600160a01b03918216600482015286936001600160701b03169291909116906370a0823190602401602060405180830381865afa158015610b57573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b7b9190610f16565b610b859190610f2d565b14610c415760c0860151606087015187516040516370a0823160e01b81526001600160a01b039182166004820152610c3e936001600160701b03169291909116906370a0823190602401602060405180830381865afa158015610bea573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c0e9190610f16565b610c189190610f2d565b8760c001516001600160701b03168860a001516001600160701b03168960800151610d65565b91505b610c4c600583610f2d565b91508415610cc5578551604080516020810182525f808252915163022c0d9f60e01b81526001600160a01b039093169263022c0d9f92610c93928792309190600401610f83565b5f604051808303815f87803b158015610caa575f80fd5b505af1158015610cbc573d5f803e3d5ffd5b50505050610d33565b8551604080516020810182525f808252915163022c0d9f60e01b81526001600160a01b039093169263022c0d9f92610d0592909187913091600401610f83565b5f604051808303815f87803b158015610d1c575f80fd5b505af1158015610d2e573d5f803e3d5ffd5b505050505b60408087015190516370a0823160e01b81523060048201525f916001600160a01b0316906370a082319060240161095d565b5f80610d718387610faf565b905080610d80866103e8610faf565b610d8a9190610efd565b610d948583610faf565b610d9e9190610fc6565b610da9906001610efd565b9695505050505050565b6001600160a01b0381168114610dc7575f80fd5b50565b5f805f60608486031215610ddc575f80fd5b8335610de781610db3565b92506020840135610df781610db3565b929592945050506040919091013590565b80516001600160701b0381168114610e1e575f80fd5b919050565b5f805f60608486031215610e35575f80fd5b610e3e84610e08565b9250610e4c60208501610e08565b9150604084015163ffffffff81168114610e64575f80fd5b809150509250925092565b5f60208284031215610e7f575f80fd5b8151610e8a81610db3565b9392505050565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b5f6001600160701b0380841680610ed257610ed2610e91565b92169190910492915050565b5f60208284031215610eee575f80fd5b81518015158114610e8a575f80fd5b80820180821115610f1057610f10610ea5565b92915050565b5f60208284031215610f26575f80fd5b5051919050565b81810381811115610f1057610f10610ea5565b5f81518084525f5b81811015610f6457602081850181015186830182015201610f48565b505f602082860101526020601f19601f83011685010191505092915050565b84815283602082015260018060a01b0383166040820152608060608201525f610da96080830184610f40565b8082028115828204841417610f1057610f10610ea5565b5f82610fd457610fd4610e91565b50049056fea26469706673582212208a70148d097f0fab5b4e8249127bc3cedb57d4e543a9437d40c5573bfda63a1964736f6c63430008140033".parse().unwrap()
}

pub fn insert_fake_approval<DB>(token: Address, pair: Address, db: &mut DB) {
    for i in 0..100 {
        let slot_new = map_location(U256::from(i), pair, tax_checker_address());
        
    }
}

pub fn map_location(slot: U256, key: Address, key_after: Address) -> U256 {
    let input = [DynSolValue::Address(key), DynSolValue::Uint(slot, 32)];
    let input = DynSolValue::Tuple(input.to_vec());
    let key_slot_hash: U256 = keccak256(input.abi_encode()).into();

    let input = [DynSolValue::Address(key_after), DynSolValue::Uint(key_slot_hash, 32)];
    let input = DynSolValue::Tuple(input.to_vec());
    let slot: U256 = keccak256(input.abi_encode()).into();
    slot
}