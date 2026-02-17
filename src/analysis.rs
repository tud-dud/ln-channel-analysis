use crate::types::{Properties, Tx};

pub(crate) fn analyse_tx(tx: Tx) -> Properties {
    let num_p2wsh_outputs = tx
        .vout
        .iter()
        .filter(|o| is_p2wsh(&o.scriptpubkey_type))
        .count();
    let num_p2tr_outputs = tx
        .vout
        .iter()
        .filter(|o| is_p2tr(&o.scriptpubkey_type))
        .count();
    let mut type_amt_funding_addresses = vec![];
    for i in &tx.vin {
        type_amt_funding_addresses.push((i.prevout.scriptpubkey_type.clone(), i.prevout.value));
    }
    let mut type_output_value = vec![];
    for i in &tx.vout {
        type_output_value.push((i.scriptpubkey_type.clone(), i.value));
    }
    Properties {
        num_outputs: tx.vout.len(),
        num_p2wsh_outputs,
        num_p2tr_outputs,
        type_amt_funding_addresses,
        type_output_value,
    }
}

fn is_p2wsh(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2wsh")
}

fn is_p2tr(scriptpubkey_type: &str) -> bool {
    scriptpubkey_type.contains("p2tr")
}
