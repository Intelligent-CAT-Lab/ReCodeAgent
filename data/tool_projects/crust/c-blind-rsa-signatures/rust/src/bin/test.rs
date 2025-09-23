use c_blind_rsa_signatures::blind_rsa::*;
#[test]
fn test_default() {
    let mut context = BRSAContext {
        evp_md: None,
        salt_len: BRSA_DEFAULT_SALT_LENGTH,
    };
    context.brsa_context_init_default();
    let mut sk = BRSASecretKey { evp_pkey: None };
    let mut pk = BRSAPublicKey { evp_pkey: None, mont_ctx: None};
    assert_eq!(sk.brsa_keypair_generate(&mut pk, 2048), 0);
    let mut msg = [0u8; 32];
    let msg_len = msg.len();
    let mut blind_msg = BRSABlindMessage { blind_message: &[], blind_message_len: 0 };
    let mut client_secret = BRSABlindingSecret { secret: &[], secret_len: 0 };
    assert_eq!(context.brsa_blind_message_generate(&mut blind_msg, &mut msg, msg_len, &mut client_secret, &mut pk), 0);
    let mut blind_sig = BRSABlindSignature { blind_sig: &[], blind_sig_len: 0 };
    assert_eq!(context.brsa_blind_sign(&mut blind_sig, &mut sk, &blind_msg), 0);
    blind_msg.brsa_blind_message_deinit();
    let mut sig = BRSASignature { sig: &[], sig_len: 0 };
    assert_eq!(context.brsa_finalize(&mut sig, &blind_sig, &client_secret, &None, &mut pk, &msg, msg_len), 0);
    blind_sig.brsa_blind_signature_deinit();
    client_secret.brsa_blinding_secret_deinit();
    assert_eq!(context.brsa_verify(&sig, &mut pk, &None, &msg, msg_len), 0);
    sig.brsa_signature_deinit();
    let mut key_id = [0u8; 4];
    assert_eq!(context.brsa_publickey_id(&key_id, key_id.len(), &pk), 0);
    let mut sk_der = BRSASerializedKey { bytes: &[], bytes_len: 0 };
    let mut pk_der = BRSASerializedKey { bytes: &[], bytes_len: 0 };
    assert_eq!(sk_der.brsa_secretkey_export(&sk), 0);
    assert_eq!(pk_der.brsa_publickey_export(&pk), 0);
    sk.brsa_secretkey_deinit();
    pk.brsa_publickey_deinit();
    assert_eq!(sk.brsa_secretkey_import(sk_der.bytes, sk_der.bytes_len), 0);
    assert_eq!(pk.brsa_publickey_import(pk_der.bytes, pk_der.bytes_len), 0);
    sk_der.brsa_serializedkey_deinit();
    pk_der.brsa_serializedkey_deinit();
    sk.brsa_secretkey_deinit();
    pk.brsa_publickey_deinit();
}
#[test]
fn test_deterministic() {
    let mut context = BRSAContext {
        evp_md: None,
        salt_len: BRSA_DEFAULT_SALT_LENGTH,
    };
    context.brsa_context_init_deterministic();
    let mut sk = BRSASecretKey { evp_pkey: None };
    let mut pk = BRSAPublicKey { evp_pkey: None, mont_ctx: None };
    assert_eq!(sk.brsa_keypair_generate(&mut pk, 2048), 0);
    let mut msg = [0u8; 32];
    let msg_len = msg.len();
    let mut blind_msg = BRSABlindMessage { blind_message: &[], blind_message_len: 0 };
    let mut client_secret = BRSABlindingSecret { secret: &[], secret_len: 0 };
    assert_eq!(context.brsa_blind_message_generate(&mut blind_msg, &msg, msg_len, &mut client_secret, &mut pk), 0);
    let mut blind_sig = BRSABlindSignature { blind_sig: &[], blind_sig_len: 0 };
    assert_eq!(context.brsa_blind_sign(&mut blind_sig, &mut sk, &blind_msg), 0);
    let mut sig = BRSASignature { sig: &[], sig_len: 0 };
    assert_eq!(context.brsa_finalize(&mut sig, &blind_sig, &client_secret, &None, &mut pk, &msg, msg_len), 0);
    client_secret.brsa_blinding_secret_deinit();
    assert_eq!(context.brsa_verify(&sig, &mut pk, &None, &msg, msg_len), 0);
    sig.brsa_signature_deinit();
    let mut blind_sig2 = BRSABlindSignature { blind_sig: &[], blind_sig_len: 0 };
    assert_eq!(context.brsa_blind_sign(&mut blind_sig2, &mut sk, &blind_msg), 0);
    blind_msg.brsa_blind_message_deinit();
    sk.brsa_secretkey_deinit();
    pk.brsa_publickey_deinit();
    assert_eq!(blind_sig.blind_sig, blind_sig2.blind_sig);
    blind_sig.brsa_blind_signature_deinit();
    blind_sig2.brsa_blind_signature_deinit();
}
#[test]
fn test_custom_parameters() {
    let mut context = BRSAContext {
        evp_md: None,
        salt_len: 48,
    };
    context.brsa_context_init_custom(BRSAHashFunction::BRSA_SHA256, 48);
    let mut sk = BRSASecretKey { evp_pkey: None };
    let mut pk = BRSAPublicKey { evp_pkey: None, mont_ctx: None };
    assert_eq!(sk.brsa_keypair_generate(&mut pk, 2048), 0);
    let mut msg = [0u8; 32];
    let msg_len = msg.len();
    let mut blind_msg = BRSABlindMessage { blind_message: &[], blind_message_len: 0 };
    let mut client_secret = BRSABlindingSecret { secret: &[], secret_len: 0 };
    assert_eq!(context.brsa_blind_message_generate(&mut blind_msg, &msg, msg_len, &mut client_secret, &mut pk), 0);
    let mut blind_sig = BRSABlindSignature { blind_sig: &[], blind_sig_len: 0 };
    assert_eq!(context.brsa_blind_sign(&mut blind_sig, &mut sk, &blind_msg), 0);
    blind_msg.brsa_blind_message_deinit();
    let mut sig = BRSASignature { sig: &[], sig_len: 0 };
    assert_eq!(context.brsa_finalize(&mut sig, &blind_sig, &client_secret, &None, &mut pk, &msg, msg_len), 0);
    blind_sig.brsa_blind_signature_deinit();
    client_secret.brsa_blinding_secret_deinit();
    assert_eq!(context.brsa_verify(&sig, &mut pk, &None, &msg, msg_len), 0);
    sig.brsa_signature_deinit();
    sk.brsa_secretkey_deinit();
    pk.brsa_publickey_deinit();
}
fn main() {}