use recordManager::buffer_mgr;
use recordManager::buffer_mgr_stat;
use recordManager::dberror;
use recordManager::dt;
use recordManager::expr;
use recordManager::record_mgr;
use recordManager::rm_serializer;
use recordManager::storage_mgr;
use recordManager::tables;

#[test]
pub fn test_value_serialize(){
    // assert that the value equals a string

    assert_eq!(rm_serializer::serialize_value(&rm_serializer::string_to_value("i10")), "i10");
    assert_eq!(rm_serializer::serialize_value(&rm_serializer::string_to_value("f5.3")), "f5.3");
    assert_eq!(rm_serializer::serialize_value(&rm_serializer::string_to_value("sHello world")), "sHello world");
    assert_eq!(rm_serializer::serialize_value(&rm_serializer::string_to_value("bt")), "bt");
    assert_eq!(rm_serializer::serialize_value(&rm_serializer::string_to_value("btrue")), "btrue");
}

#[test]
pub fn testOperators(){
    // eqality
    let mut result = tables::make_value(&tables::DataType::DtInt.to_string(), "0");
    assert_eq!(expr::value_equals(&rm_serializer::string_to_value("i10"), &rm_serializer::string_to_value("i10"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::value_equals(&rm_serializer::string_to_value("i9"), &rm_serializer::string_to_value("i10"), &mut result), dberror::RC::Ok);
    let mut result = tables::make_value(&tables::DataType::DtString.to_string(), "0");
    assert_eq!(expr::value_equals(&rm_serializer::string_to_value("sHello World"), &rm_serializer::string_to_value("sHello World"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::value_equals(&rm_serializer::string_to_value("sHello Worl"), &rm_serializer::string_to_value("sHello World"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::value_equals(&rm_serializer::string_to_value("sHello Worl"), &rm_serializer::string_to_value("sHello Wor"), &mut result), dberror::RC::Ok);
    // smaller
    assert_eq!(expr::value_smaller(&rm_serializer::string_to_value("i3"), &rm_serializer::string_to_value("i10"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::value_smaller(&rm_serializer::string_to_value("f5.0"), &rm_serializer::string_to_value("f6.5"), &mut result), dberror::RC::Ok);
    // boolean
    assert_eq!(expr::bool_and(&rm_serializer::string_to_value("bt"), &rm_serializer::string_to_value("bt"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::bool_and(&rm_serializer::string_to_value("bf"), &rm_serializer::string_to_value("bt"), &mut result), dberror::RC::Ok);

    assert_eq!(expr::bool_or(&rm_serializer::string_to_value("bt"), &rm_serializer::string_to_value("bf"), &mut result), dberror::RC::Ok);
    assert_ne!(expr::bool_or(&rm_serializer::string_to_value("bf"), &rm_serializer::string_to_value("bf"), &mut result), dberror::RC::Ok);

    assert_eq!(expr::bool_not(&rm_serializer::string_to_value("bt"), &mut result), dberror::RC::Ok);

}

#[test]
fn test_expressions() {
    let mut res = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    };

    println!("Running test: test complex expressions");

    // Creating a constant value expression (10)
    let l = expr::Expr {
        expr_type: expr::ExprType::ExprConst,
        expr: expr::ExprUnion::Cons(rm_serializer::string_to_value("i10")),
    };
    expr::eval_expr(&tables::Record { id: tables::RID { page: 0, slot: 0 }, data: String::new() }, &tables::Schema {
        num_attr: 0,
        attr_names: vec![],
        data_types: vec![],
        type_length: vec![],
        key_attrs: vec![],
        key_size: 0,
    }, &l, &mut res);
    let mut res2 = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    }; 
    assert!(expr::value_equals(&rm_serializer::string_to_value("i10"), &res, &mut res2) == dberror::RC::Ok, "Const 10");

    // Creating a constant value expression (20)
    let r = expr::Expr {
        expr_type: expr::ExprType::ExprConst,
        expr: expr::ExprUnion::Cons(rm_serializer::string_to_value("i20")),
    };
    expr::eval_expr(&tables::Record { id: tables::RID { page: 0, slot: 0 }, data: String::new() }, &tables::Schema {
        num_attr: 0,
        attr_names: vec![],
        data_types: vec![],
        type_length: vec![],
        key_attrs: vec![],
        key_size: 0,
    }, &r, &mut res);
    let mut res2 = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    }; 
    assert!(expr::value_equals(&rm_serializer::string_to_value("i20"), &res, &mut res2) == dberror::RC::Ok, "Const 20");

    // Creating a binary operation expression (10 < 20)
    let op = expr::Expr {
        expr_type: expr::ExprType::ExprOp,
        expr: expr::ExprUnion::Op(Box::new(expr::Operator {
            op_type: expr::OpType::OpCompSmaller,
            args: vec![l.clone(), r.clone()],
        })),
    };
    expr::eval_expr(&tables::Record { id: tables::RID { page: 0, slot: 0 }, data: String::new() }, &tables::Schema {
        num_attr: 0,
        attr_names: vec![],
        data_types: vec![],
        type_length: vec![],
        key_attrs: vec![],
        key_size: 0,
    }, &op, &mut res);
    let mut res2 = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    }; 
    assert!(expr::value_equals(&rm_serializer::string_to_value("bt"), &res, &mut res2) == dberror::RC::Ok, "Const 10 < Const 20");

    // Creating a constant boolean value expression (true)
    let l = expr::Expr {
        expr_type: expr::ExprType::ExprConst,
        expr: expr::ExprUnion::Cons(rm_serializer::string_to_value("bt")),
    };
    expr::eval_expr(&tables::Record { id: tables::RID { page: 0, slot: 0 }, data: String::new() }, &tables::Schema {
        num_attr: 0,
        attr_names: vec![],
        data_types: vec![],
        type_length: vec![],
        key_attrs: vec![],
        key_size: 0,
    }, &l, &mut res);
    let mut res2 = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    }; 
    assert!(expr::value_equals(&rm_serializer::string_to_value("bt"), &res, &mut res2) == dberror::RC::Ok, "Const true");

    // Logical AND: (10 < 20) AND true
    let op = expr::Expr {
        expr_type: expr::ExprType::ExprOp,
        expr: expr::ExprUnion::Op(Box::new(expr::Operator {
            op_type: expr::OpType::OpBoolAnd,
            args: vec![op.clone(), l.clone()],
        })),
    };
    expr::eval_expr(&tables::Record { id: tables::RID { page: 0, slot: 0 }, data: String::new() }, &tables::Schema {
        num_attr: 0,
        attr_names: vec![],
        data_types: vec![],
        type_length: vec![],
        key_attrs: vec![],
        key_size: 0,
    }, &op, &mut res);
    let mut res2 = tables::Value {
        dt: tables::DataType::DtInt, // Placeholder, will be overwritten
        v: tables::ValueUnion::IntV(0),
    }; 
    assert!(expr::value_equals(&rm_serializer::string_to_value("bt"), &res, &mut res2) == dberror::RC::Ok, "(Const 10 < Const 20) AND true");

    println!("Test Done.");
}
fn main(){}