use crate::{
    ast::{
        AssignmentExpression, BinaryExpression, CallExpression, Callee, IfStatement,
        ProcedureDeclaration, Statement, TestType, WhileStatement,
    },
    commands::{Literal, Value},
    program::Program,
    tokens::{
        AssignmentType, CalculationOperation, CommandType, IdentifierType, LogicalOperation,
        QueryType, TokenType, ValueType,
    },
};

use super::Image;

fn init_program_code(code: &str, run: bool) -> Program {
    let image = Image::new(200, 200);
    let mut program = Program::parse_logo(&code.to_string(), image);
    if run {
        program.run();
    }
    program
}
fn build_control_call_expression(command: CommandType, args: Vec<Value>) -> Statement {
    Statement::CallExpression(CallExpression::new(Callee::Command(command), args))
}
fn build_query_expression(query: QueryType) -> Statement {
    Statement::CallExpression(CallExpression::new(
        Callee::Query(query.clone()),
        Vec::new(),
    ))
}
fn build_variable_assignment_expression(
    operation: AssignmentType,
    left: String,
    right: Value,
) -> Statement {
    Statement::AssignmentExpression(AssignmentExpression::new(operation, left, right))
}
fn compare_statement(statements: &Statement, expected: Statement) {
    assert_eq!(
        serde_json::to_string(statements).unwrap(),
        serde_json::to_string(&expected).unwrap()
    );
}
fn build_value_from_query(query_type: QueryType) -> Value {
    Value::Statement(Box::new(
        build_query_expression(query_type)
            .expect_callexpression()
            .unwrap(),
    ))
}
fn build_if_statement(condition: TestType, body: Vec<Statement>) -> Statement {
    Statement::If(IfStatement::new(condition, body))
}
fn build_while_statement(condition: TestType, body: Vec<Statement>) -> Statement {
    Statement::While(WhileStatement::new(condition, body))
}

fn build_value_from_binary_expression(operation: TokenType, left: Value, right: Value) -> Value {
    Value::BinaryExpression(Box::new(BinaryExpression::new(operation, left, right)))
}
fn build_procedure_call_expression(procedure_name: String, args: Vec<Value>) -> Statement {
    Statement::CallExpression(CallExpression::new(Callee::Procedure(procedure_name), args))
}
fn build_procedure_declaration(
    procedure_name: String,
    args: Vec<String>,
    body: Vec<Statement>,
) -> Statement {
    Statement::ProcedureDeclaration(ProcedureDeclaration::new(procedure_name, args, body))
}

//* ****************************************************************************** //
//*                                                                                //
//*                             AST Parsing Tests                                  //
//*                                                                                //
//* ****************************************************************************** //

#[test]
fn test_empty_file() {
    let code = "

    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 0);
}

#[test]
fn test_parse_comments_only_file() {
    let code = "
    // something
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 0);
}
#[test]
fn test_parse_turtle_control() {
    let code = "
    // Control command can parse with constant value
    PENUP
    PENDOWN
    FORWARD \"10
    BACK \"10
    LEFT \"10
    RIGHT \"10
    SETPENCOLOR \"10
    SETHEADING \"10
    SETX \"10
    SETY \"10
    TURN \"10
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 11);
    let const_value = Value::Literal(ValueType::Int(10));
    compare_statement(
        &program.statements()[0],
        build_control_call_expression(CommandType::Penup, vec![]),
    );
    compare_statement(
        &program.statements()[1],
        build_control_call_expression(CommandType::Pendown, vec![]),
    );
    compare_statement(
        &program.statements()[2],
        build_control_call_expression(CommandType::Forward, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[3],
        build_control_call_expression(CommandType::Back, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[4],
        build_control_call_expression(CommandType::Left, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[5],
        build_control_call_expression(CommandType::Right, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[6],
        build_control_call_expression(CommandType::Setpencolor, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[7],
        build_control_call_expression(CommandType::Setheading, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[8],
        build_control_call_expression(CommandType::Setx, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[9],
        build_control_call_expression(CommandType::Sety, vec![const_value.clone()]),
    );
    compare_statement(
        &program.statements()[10],
        build_control_call_expression(CommandType::Turn, vec![const_value.clone()]),
    );
}

#[test]
fn test_parse_variables() {
    let code = "
    // MAKE/ADDASSIGN can parse with constant value or variable value.

    // with constant value
    MAKE \"variable \"10
    ADDASSIGN \"variable \"10

    // with variable value
    ADDASSIGN \"variable :variable
    MAKE \"variable :variable
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 4);
    let variable_name = "variable".to_string();
    let const_value_10 = Value::Literal(ValueType::Int(10));
    let variable_right_value = Value::Identifier(IdentifierType::Variable(variable_name.clone()));
    compare_statement(
        &program.statements()[0],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            const_value_10.clone(),
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_variable_assignment_expression(
            AssignmentType::Addassign,
            variable_name.clone(),
            const_value_10.clone(),
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_variable_assignment_expression(
            AssignmentType::Addassign,
            variable_name.clone(),
            variable_right_value.clone(),
        ),
    );
    compare_statement(
        &program.statements()[3],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            variable_right_value.clone(),
        ),
    );
}

#[test]
fn test_parse_controls_with_variable() {
    let code = "
    // Control command can parse with variable value.
    // after make
    MAKE \"variable \"10
    FORWARD :variable

    // after addassign
    ADDASSIGN \"variable \"10
    FORWARD :variable

    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 4);
    let variable_name = "variable".to_string();
    let const_value_10 = Value::Literal(ValueType::Int(10));
    let variable_right_value = Value::Identifier(IdentifierType::Variable(variable_name.clone()));
    compare_statement(
        &program.statements()[0],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            const_value_10.clone(),
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_control_call_expression(CommandType::Forward, vec![variable_right_value.clone()]),
    );
    compare_statement(
        &program.statements()[2],
        build_variable_assignment_expression(
            AssignmentType::Addassign,
            variable_name.clone(),
            const_value_10.clone(),
        ),
    );
    compare_statement(
        &program.statements()[3],
        build_control_call_expression(CommandType::Forward, vec![variable_right_value.clone()]),
    );
}

#[test]
fn test_parse_all_queries() {
    let code = "
    // All queries can parse 
    MAKE \"variable XCOR
    MAKE \"variable YCOR
    MAKE \"variable HEADING
    MAKE \"variable COLOR
    
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 4);
    let variable_name = "variable".to_string();
    compare_statement(
        &program.statements()[0],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_query(QueryType::Xcor),
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_query(QueryType::Ycor),
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_query(QueryType::Heading),
        ),
    );
    compare_statement(
        &program.statements()[3],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_query(QueryType::Color),
        ),
    );
}
#[test]
fn test_parse_query_as_variable() {
    let code = "
    // Queries can parse and use as a variable
    MAKE \"variable XCOR
    // Queries can parse and use in control command
    FORWARD XCOR
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 2);
    let variable_name = "variable".to_string();
    let right_value = build_value_from_query(QueryType::Xcor);
    compare_statement(
        &program.statements()[0],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            right_value.clone(),
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_control_call_expression(CommandType::Forward, vec![right_value.clone()]),
    );
}

#[test]
fn test_parse_simple_binary() {
    let code = "
    // test simple binary operation
    // can be put into variable
    MAKE \"variable EQ \"10 \"10 
    MAKE \"variable NE \"10 \"10 
    MAKE \"variable GT \"10 \"10 
    MAKE \"variable LT \"10 \"10 
    MAKE \"variable AND \"TRUE \"TRUE 
    MAKE \"variable OR \"TRUE \"TRUE 
    MAKE \"variable + \"10 \"10 
    MAKE \"variable - \"10 \"10 
    MAKE \"variable * \"10 \"10 
    MAKE \"variable / \"10 \"10 

    // can be put into control command as a variable
    FORWARD + \"10 \"10 

    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 11);
    let consts_value = Value::Literal(ValueType::Int(10));
    let consts_bool = Value::Literal(ValueType::Bool(true));
    let variable_name = "variable".to_string();
    compare_statement(
        &program.statements()[0],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Eq),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Ne),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Gt),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[3],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Lt),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[4],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::And),
                consts_bool.clone(),
                consts_bool.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[5],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Or),
                consts_bool.clone(),
                consts_bool.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[6],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Calculation(CalculationOperation::Plus),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[7],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Calculation(CalculationOperation::Dash),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[8],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Calculation(CalculationOperation::Star),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[9],
        build_variable_assignment_expression(
            AssignmentType::Make,
            variable_name.clone(),
            build_value_from_binary_expression(
                TokenType::Calculation(CalculationOperation::Slash),
                consts_value.clone(),
                consts_value.clone(),
            ),
        ),
    );
    compare_statement(
        &program.statements()[10],
        build_control_call_expression(
            CommandType::Forward,
            vec![build_value_from_binary_expression(
                TokenType::Calculation(CalculationOperation::Plus),
                consts_value.clone(),
                consts_value.clone(),
            )],
        ),
    );
}

#[test]
fn test_parse_long_binary() {
    let code = "
    // test complex binary operation
    MAKE \"variable EQ EQ \"10 \"10 EQ \"10 \"10
    ";
    let program = init_program_code(code, false);
    let consts_value = Value::Literal(ValueType::Int(10));
    let sub_binary = build_value_from_binary_expression(
        TokenType::Logic(LogicalOperation::Eq),
        consts_value.clone(),
        consts_value.clone(),
    );
    let expected = build_variable_assignment_expression(
        AssignmentType::Make,
        "variable".to_string(),
        build_value_from_binary_expression(
            TokenType::Logic(LogicalOperation::Eq),
            sub_binary.clone(),
            sub_binary.clone(),
        ),
    );
    compare_statement(&program.statements()[0], expected);
}

#[test]
fn test_parse_if() {
    let code = "
    // parse if statement
    IF \"TRUE [
        FORWARD \"10
    ]
    IF EQ \"10 \"10 [
        FORWARD \"10
    ]
    IF NE XCOR \"10 [
        FORWARD \"10
    ]
    
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 3);
    let consts_value = Value::Literal(ValueType::Int(10));
    compare_statement(
        &program.statements()[0],
        build_if_statement(
            Value::Literal(ValueType::Bool(true)),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_if_statement(
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Eq),
                consts_value.clone(),
                consts_value.clone(),
            ),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_if_statement(
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Ne),
                build_value_from_query(QueryType::Xcor),
                consts_value.clone(),
            ),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
}

#[test]
fn test_parse_while() {
    let code = "
    // parse while statement
    WHILE \"TRUE [
        FORWARD \"10
    ]
    WHILE EQ \"10 \"10 [
        FORWARD \"10
    ]
    WHILE NE \"10 \"10 [
        FORWARD \"10
    ]
    WHILE NE XCOR \"10 [
        FORWARD \"10
    ]
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 4);
    let consts_value = Value::Literal(ValueType::Int(10));
    compare_statement(
        &program.statements()[0],
        build_while_statement(
            Value::Literal(ValueType::Bool(true)),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_while_statement(
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Eq),
                consts_value.clone(),
                consts_value.clone(),
            ),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_while_statement(
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Ne),
                consts_value.clone(),
                consts_value.clone(),
            ),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
    compare_statement(
        &program.statements()[3],
        build_while_statement(
            build_value_from_binary_expression(
                TokenType::Logic(LogicalOperation::Ne),
                build_value_from_query(QueryType::Xcor),
                consts_value.clone(),
            ),
            vec![build_control_call_expression(
                CommandType::Forward,
                vec![consts_value.clone()],
            )],
        ),
    );
}

#[test]
fn test_parse_nested_if_while() {
    let code = "
    // parse while statement
    WHILE \"TRUE [
        FORWARD \"10
        WHILE EQ \"10 \"10 [
            FORWARD \"10
            IF NE XCOR \"10 [
                FORWARD \"10
                IF EQ \"10 \"10 [
                    FORWARD \"10
                ]
            ]
        ]
    ]
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 1);
    let consts_value = Value::Literal(ValueType::Int(10));
    compare_statement(
        &program.statements()[0],
        build_while_statement(
            Value::Literal(ValueType::Bool(true)),
            vec![
                build_control_call_expression(CommandType::Forward, vec![consts_value.clone()]),
                build_while_statement(
                    build_value_from_binary_expression(
                        TokenType::Logic(LogicalOperation::Eq),
                        consts_value.clone(),
                        consts_value.clone(),
                    ),
                    vec![
                        build_control_call_expression(
                            CommandType::Forward,
                            vec![consts_value.clone()],
                        ),
                        build_if_statement(
                            build_value_from_binary_expression(
                                TokenType::Logic(LogicalOperation::Ne),
                                build_value_from_query(QueryType::Xcor),
                                consts_value.clone(),
                            ),
                            vec![
                                build_control_call_expression(
                                    CommandType::Forward,
                                    vec![consts_value.clone()],
                                ),
                                build_if_statement(
                                    build_value_from_binary_expression(
                                        TokenType::Logic(LogicalOperation::Eq),
                                        consts_value.clone(),
                                        consts_value.clone(),
                                    ),
                                    vec![build_control_call_expression(
                                        CommandType::Forward,
                                        vec![consts_value.clone()],
                                    )],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        ),
    );
}

#[test]
fn test_parse_procedure() {
    let code = "
    // parse procedure
    TO BOX 
        FORWARD \"10
        FORWARD \"10
    END

    TO BOXX \"Arg
        FORWARD :Arg
        FORWARD :Arg
    END

    BOX 
    BOXX \"10
    ";
    let program = init_program_code(code, false);
    assert_eq!(program.statements().len(), 4);
    let consts_value = Value::Literal(ValueType::Int(10));
    compare_statement(
        &program.statements()[0],
        build_procedure_declaration(
            "BOX".to_string(),
            vec![],
            vec![
                build_control_call_expression(CommandType::Forward, vec![consts_value.clone()]),
                build_control_call_expression(CommandType::Forward, vec![consts_value.clone()]),
            ],
        ),
    );
    compare_statement(
        &program.statements()[1],
        build_procedure_declaration(
            "BOXX".to_string(),
            vec!["Arg".to_string()],
            vec![
                build_control_call_expression(
                    CommandType::Forward,
                    vec![Value::Identifier(IdentifierType::Variable(
                        "Arg".to_string(),
                    ))],
                ),
                build_control_call_expression(
                    CommandType::Forward,
                    vec![Value::Identifier(IdentifierType::Variable(
                        "Arg".to_string(),
                    ))],
                ),
            ],
        ),
    );
    compare_statement(
        &program.statements()[2],
        build_procedure_call_expression("BOX".to_string(), vec![]),
    );
    compare_statement(
        &program.statements()[3],
        build_procedure_call_expression(
            "BOXX".to_string(),
            vec![Value::Literal(ValueType::Int(10))],
        ),
    );
}
//* ****************************************************************************** //
//*                                                                                //
//*                             AST Execution Tests                                //
//*                                                                                //
//* ****************************************************************************** //

fn compare_turtle_position(program: &Program, expected: (f32, f32), abs: bool) {
    let turtle = program.heap().turtle();
    if abs {
        assert_eq!(turtle.x(), expected.0);
        assert_eq!(turtle.y(), expected.1);
    } else {
        assert_eq!(turtle.x(), expected.1 + 100.0);
        assert_eq!(turtle.y(), expected.0 * -1.0 + 100.0);
    }
}
fn compare_turtle_direction(program: &Program, expected: i32) {
    let turtle = program.heap().turtle();
    assert_eq!(turtle.direction(), expected);
}
fn compare_turtle_color(program: &Program, expected: i32) {
    let turtle = program.heap().turtle();
    assert_eq!(turtle.color(), expected);
}
fn compare_turtle_pen(program: &Program, expected: bool) {
    let turtle = program.heap().turtle();
    assert_eq!(turtle.pen(), expected);
}
fn compare_vairable_value(program: &Program, variable_name: &str, expected: Literal) {
    let heap = program.heap();
    let variable = heap
        .variable_value(&variable_name.to_string())
        .expect("Variable is not found");
    assert_eq!(variable, expected);
}

#[test]
fn test_turtle_move() {
    let code = "
    PENDOWN
    FORWARD \"10
    BACK \"20
    LEFT \"10
    RIGHT \"20
    TURN \"90
    ";
    let mut program = init_program_code(code, false);
    compare_turtle_pen(&program, false);
    compare_turtle_position(&program, (0.0, 0.0), false);
    program.run();
    compare_turtle_position(&program, (-10.0, 10.0), false);
    compare_turtle_direction(&program, 90);
    compare_turtle_pen(&program, true);
}

#[test]
fn test_turtle_set() {
    let code = "
    PENDOWN
    FORWARD \"10
    BACK \"20
    LEFT \"10
    RIGHT \"20
    TURN \"90
    SETPENCOLOR \"2
    SETHEADING \"10
    SETX \"90
    SETY \"90
    ";
    let program = init_program_code(code, true);
    compare_turtle_direction(&program, 10);
    compare_turtle_color(&program, 2);
    compare_turtle_position(&program, (90.0, 90.0), true);
}

#[test]
fn test_variables() {
    let code = "
    MAKE \"variable \"10
    ADDASSIGN \"variable \"10
    FORWARD :variable
    ";
    let program = init_program_code(code, true);
    compare_vairable_value(&program, "variable", Literal::Int(20));
}

#[test]
fn test_query() {
    let code = "
    SETX \"100
    SETY \"100
    SETHEADING \"100
    SETPENCOLOR \"0
    MAKE \"x XCOR
    MAKE \"y YCOR
    MAKE \"heading HEADING
    MAKE \"color COLOR
    ";
    let program = init_program_code(code, true);
    compare_vairable_value(&program, "x", Literal::Float(100.0));
    compare_vairable_value(&program, "y", Literal::Float(100.0));
    compare_vairable_value(&program, "color", Literal::Int(0));
    compare_vairable_value(&program, "heading", Literal::Int(100));
}

#[test]
fn test_query_variable_move() {
    let code = "
    SETX \"0
    MAKE \"color XCOR
    SETY \"0
    FORWARD YCOR
    SETPENCOLOR :color
    FORWARD YCOR
    ";
    let program = init_program_code(code, true);
    compare_vairable_value(&program, "color", Literal::Float(0.0));
    compare_turtle_position(&program, (0.0, 0.0), true);
    compare_turtle_color(&program, 0);
}
#[test]
fn test_if_while() {
    let code = "
    MAKE \"i \"0
    WHILE LT :i \"10 [
        FORWARD \"10
        IF EQ :i \"5 [
            LEFT \"10
        ]
        ADDASSIGN \"i \"1
    ]
    ";
    let program = init_program_code(code, true);
    compare_turtle_position(&program, (90.0, 0.0), true);
}
#[test]
fn test_math() {
    let code = "
    MAKE \"eq EQ \"10 \"10 
    MAKE \"ne NE \"10 \"10 
    MAKE \"gt GT \"10 \"10 
    MAKE \"lt LT \"10 \"10 
    MAKE \"and AND \"TRUE \"TRUE 
    MAKE \"or OR \"TRUE \"TRUE 
    MAKE \"or_2 OR \"TRUE \"FALSE 
    MAKE \"plus + \"10 \"10 
    MAKE \"sub - \"10 \"10 
    MAKE \"mul * \"10 \"10 
    MAKE \"div / \"10 \"10 
    MAKE \"long1 EQ EQ \"10 \"10 EQ \"10 \"10
    MAKE \"long2 + + \"10 \"10 + \"10 \"10
    MAKE \"long3 + + \"10 :mul + \"10 XCOR

    ";
    let program = init_program_code(code, true);
    compare_vairable_value(&program, "eq", Literal::Bool(true));
    compare_vairable_value(&program, "ne", Literal::Bool(false));
    compare_vairable_value(&program, "gt", Literal::Bool(false));
    compare_vairable_value(&program, "lt", Literal::Bool(false));
    compare_vairable_value(&program, "and", Literal::Bool(true));
    compare_vairable_value(&program, "or", Literal::Bool(true));
    compare_vairable_value(&program, "or_2", Literal::Bool(true));
    compare_vairable_value(&program, "plus", Literal::Int(20));
    compare_vairable_value(&program, "sub", Literal::Int(0));
    compare_vairable_value(&program, "mul", Literal::Int(100));
    compare_vairable_value(&program, "div", Literal::Int(1));
    compare_vairable_value(&program, "long1", Literal::Bool(true));
    compare_vairable_value(&program, "long2", Literal::Int(40));
    compare_vairable_value(&program, "long3", Literal::Int(220));
}

#[test]
fn test_procedure() {
    let code = "
    MAKE \"Arg \"90
    TO BOX 
        FORWARD \"10
        FORWARD \"10
    END

    TO BOXX \"Arg
        FORWARD :Arg
        FORWARD :Arg
    END

    BOX 
    BOXX \"50


    ";
    let program = init_program_code(code, true);
    compare_turtle_position(&program, (100.0, -20.0), true);
    compare_vairable_value(&program, "Arg", Literal::Int(90));
}
