#include "parser.hpp"

#include <cmath>
#include <functional>
#include <vector>
#include <iostream>
#include <fstream>
#include <memory>

#include "lexer.hpp"
#include "tokens.hpp"

parser::parser(std::vector<lexeme> &lexemes)
	: m_lexemes(lexemes)
{
	m_it = m_lexemes.begin();
	current_function = nullptr;
}

parser::parser(const parser &p) : m_lexemes(p.m_lexemes)
{
}

void parser::write_line(std::ofstream &of, parsed_item p)
{
	of << "File " << p.infile_name;
	of << " ";
	of << "Line " << p.lineno;
	of << ": ";
	switch (p.k)
	{
	case GLOBAL_VARIABLE:
		of << "global variable ";
		break;
	case GLOBAL_STRUCT:
		of << "global struct ";
		break;
	case FUNCTION:
		of << "function ";
		break;
	case PARAMETER:
		of << "parameter ";
		break;
	case LOCAL_VARIABLE:
		of << "local variable ";
		break;
	case LOCAL_STRUCT:
		of << "local struct";
		break;
	case MEMBER:
		of << "member ";
		break;
	}

	of << p.ident;

	of << std::endl;
}

void write_type(std::ofstream &of, expression_t *e)
{
	if (e->is_return_expression())
	{
		return;
	}

	of << "File " << e->op.infile_name;
	of << " ";
	of << "Line " << e->op.lineno;
	of << ": expression has type ";
	switch (e->derived_type)
	{
	case parser_type::INT:
		of << "int";
		break;
	case parser_type::CHAR:
		of << "char";
		break;
	case parser_type::FLOAT:
		of << "float";
		break;
	case parser_type::VOID:
		of << "void";
		break;
	default:
		of << "error";
		break;
	}

	if (e->is_array)
	{
		of << "[]";
	}

	of << std::endl;
}

void parser::generate_output(std::string &outfile_path)
{
	std::ofstream outfile(outfile_path);

	if (!outfile)
	{
		std::string error = "\tCouldn't open file for output: ";
		error += outfile_path;

		throw error;
	}

	for (parsed_item &p : m_parsed_items)
	{
		write_line(outfile, p);
	}

	outfile.close();
}

void parser::generate_type_output(std::string &outfile_path)
{
	std::ofstream outfile(outfile_path);

	if (!outfile)
	{
		std::string error = "\tCouldn't open file for output: ";
		error += outfile_path;

		throw error;
	}

	for (function_t *f : m_functions)
	{
		for (expression_t *e : f->block.stmts)
		{
			write_type(outfile, e);
		}
	}

	outfile.close();
}

inline void parser::add_identifier(kind k, std::string &i)
{
	parsed_item p;
	p.infile_name = next().infile_name;
	p.lineno = next().lineno;
	p.k = k;
	p.ident = i;

	m_parsed_items.push_back(p);
}

inline void parser::add_identifier(kind k)
{
	parsed_item p;
	p.infile_name = next().infile_name;
	p.lineno = next().lineno;
	p.k = k;
	p.ident = next().lex;

	m_parsed_items.push_back(p);
}

void parser::error_expected(const char *expected)
{
	std::cerr << "Parser error in file " << next().infile_name << " line " << next().lineno << " at text " << next().lex << std::endl;
	std::cerr << "\tExpected '" << expected << "'" << std::endl;

	exit(1);
}

int parser::process()
{
	while (next().token != END)
	{
		if (next().token != tokentype::TYPE)
		{
			error_expected("function or global declaration");
		}

		std::string type = next().lex;
		m_it++;

		if (next().token != tokentype::IDENT)
		{
			error_expected("identifier");
		}

		std::string identifier = next().lex;
		m_it++;

		if (next().token == tokentype::LPAR) // function declaration
		{
			add_identifier(FUNCTION, identifier);
			m_it++;

			function_t *f = new function_t(type, identifier, next());
			current_function = f;

			function_definition(f);
			m_functions.push_back(f);
		}
		else
		{
			variable_t *v = new variable_t(type, identifier, next(), false);
			add_identifier(GLOBAL_VARIABLE, identifier);

			m_global_variables.push_back(v);
			variable_declaration(GLOBAL_VARIABLE, nullptr, v);
		}
	}

	return 0;
}

int parser::variable_declaration(kind k, function_t *f, variable_t *last)
{
	variable_t *v;

	if (next().token == SEMI)
	{
		m_it++;

		return 0;
	}

	if (next().token == COMMA)
	{
		m_it++;

		if (next().token != IDENT)
		{
			error_expected("identifier");
		}

		lexeme l = next();
		v = new variable_t(last->type, l.lex, l, false);

		add_identifier(k);
		m_it++;

		if (k == GLOBAL_VARIABLE)
		{
			m_global_variables.push_back(v);
		}
		else
		{
			f->local_variables.push_back(v);
		}

		variable_declaration(k, f, v);

		return 0;
	}

	if (next().token == LBRAK)
	{
		last->is_array = true;
		m_it++;

		if (next().token != INT_LIT)
		{
			error_expected("integer literal");
		}

		m_it++;

		if (next().token != RBRAK)
		{
			error_expected("]");
		}

		m_it++;

		variable_declaration(k, f, last);

		return 0;
	}

	error_expected(";");

	return 1;
}

// starting with formal_parameters
int parser::function_declaration(function_t *f)
{
	formal_parameters(f);

	if (next().token != tokentype::RPAR)
	{
		error_expected(")");
	}

	m_it++;

	return 0;
}

int parser::formal_parameters(function_t *f)
{
	variable_t *p;

	while (next().token != tokentype::RPAR)
	{
		std::string type = next().lex;
		if (next().token != tokentype::TYPE)
		{
			error_expected("type");
		}

		m_it++;

		if (next().token != IDENT)
		{
			error_expected("identifier");
		}

		p = new variable_t(type, next().lex, next(), false);

		f->params.push_back(p);
		add_identifier(PARAMETER);
		m_it++;

		if (next().token == tokentype::LBRAK)
		{
			p->is_array = true;
			m_it++;

			if (next().token != tokentype::RBRAK)
			{
				error_expected("]");
			}

			m_it++;

			if (next().token == tokentype::COMMA)
			{
				m_it++;

				continue;
			}
		}
		else if (next().token == tokentype::COMMA)
		{
			m_it++;

			continue;
		}
	}

	return 0;
}

// could be a prototype also
int parser::function_definition(function_t *f)
{
	function_declaration(f);

	// if (next().token == SEMI)
	// {
	// 	f->prototype = true;
	// 	m_it++;

	// 	return 0;
	// }
	// f->prototype = false;

	if (next().token != LBRACE)
	{
		error_expected("{");
	}

	m_it++;
	statements(&f->block);

	if (next().token != RBRACE)
	{
		error_expected("}");
	}

	m_it++;

	return 0;
}

int parser::statements(block_expression_t *block)
{
	break_expression_t *sb;
	continue_expression_t *sc;
	return_expression_t *sr;
	if_expression_t *sif;
	for_expression_t *sfor;
	while_expression_t *swhile;
	do_expression_t *sdo;
	expression_t *s;

	variable_t *v;

	while (next().token != RBRACE)
	{
		std::string type = next().lex;
		lexeme op = next();
		switch (op.token)
		{
		case SEMI:
			m_it++;
			return 0;

		case BREAK:
			sb = new break_expression_t;
			sb->op = op;
			block->push_back(sb);

			m_it++;

			if (next().token == SEMI)
			{
				m_it++;

				continue;
			}

			error_expected(";");
		case CONTINUE:
			sc = new continue_expression_t;
			sc->op = op;
			block->push_back(sc);

			m_it++;

			if (next().token == SEMI)
			{
				m_it++;

				continue;
			}

			error_expected(";");
		case RETURN:
			sr = new return_expression_t;
			sr->op = op;
			block->push_back(sr);

			m_it++;

			if (next().token == SEMI)
			{
				m_it++;

				continue;
			}

			sr->set_expression(expression_p());

			if (next().token == SEMI)
			{
				m_it++;

				continue;
			}

			error_expected(";");
		case IF:
			sif = new if_expression_t;
			sif->op = op;
			block->push_back(sif);

			m_it++;

			if_p(sif);

			continue;
		case FOR:
			sfor = new for_expression_t;
			sfor->op = op;
			block->push_back(sfor);

			m_it++;

			for_p(sfor);

			continue;
		case WHILE:
			swhile = new while_expression_t;
			swhile->op = op;
			block->push_back(swhile);

			m_it++;

			while_p(swhile);

			continue;
		case DO:
			sdo = new do_expression_t;
			sdo->op = op;
			block->push_back(sdo);

			m_it++;

			do_p(sdo);

			continue;
		case TYPE:
			m_it++;
			if (next().token != IDENT)
			{
				error_expected("identifier");
			}

			add_identifier(LOCAL_VARIABLE);

			v = new variable_t(op.lex, next().lex, next(), false);
			m_it++;

			current_function->local_variables.push_back(v);
			variable_declaration(LOCAL_VARIABLE, current_function, v);

			continue;
		default:
			s = expression_p();
			if (s)
			{
				block->push_back(s);
			}

			if (next().token != SEMI)
			{
				error_expected(";");
			}

			m_it++;

			continue;
		}
	}

	return 0;
}

block_expression_t *parser::statement_or_block(block_expression_t *sb)
{
	if (next().token == LBRACE)
	{
		m_it++;

		statement_p(sb);

		return sb;
	}

	sb->push_back(statement());

	return sb;
}

int parser::statement_p(block_expression_t *sb)
{
	while (next().token != RBRACE)
	{
		statements(sb);
	}

	m_it++;

	return 0;
}

expression_t *parser::statement()
{
	break_expression_t *sb;
	continue_expression_t *sc;
	return_expression_t *sr;
	if_expression_t *sif;
	for_expression_t *sfor;
	while_expression_t *swhile;
	do_expression_t *sdo;
	expression_t *s;

	switch (next().token)
	{
	case SEMI:
		m_it++;

		return nullptr;

	case BREAK:
		sb = new break_expression_t;
		sb->op = next();

		m_it++;

		if (next().token == SEMI)
		{
			m_it++;

			return sb;
		}

		error_expected(";");
	case CONTINUE:
		sc = new continue_expression_t;

		m_it++;

		if (next().token == SEMI)
		{
			m_it++;

			return sc;
		}

		error_expected(";");
	case RETURN:
		sr = new return_expression_t;

		m_it++;

		if (next().token == SEMI)
		{
			m_it++;

			return sr;
		}

		sr->set_expression(expression_p());

		if (next().token == SEMI)
		{
			m_it++;

			return sr;
		}

		error_expected(";");
	case IF:
		sif = new if_expression_t;

		m_it++;
		if_p(sif);

		return sif;
	case FOR:
		sfor = new for_expression_t;

		m_it++;
		for_p(sfor);

		return sfor;
	case WHILE:
		swhile = new while_expression_t;

		m_it++;
		while_p(swhile);

		return swhile;
	case DO:
		sdo = new do_expression_t;

		m_it++;
		do_p(sdo);

		return sdo;
	default:
		s = expression_p();

		if (next().token != SEMI)
		{
			error_expected(";");
		}

		m_it++;

		return s;
	}
}

void parser::if_p(if_expression_t *sif)
{
	expression_t *e;

	if (next().token != LPAR)
	{
		error_expected("(");
	}

	m_it++;

	e = expression_p();

	if (next().token != RPAR)
	{
		error_expected(")");
	}

	m_it++;

	statement_or_block(sif);

	if (next().token == ELSE)
	{
		m_it++;

		statement_or_block(sif); // TODO: fix this, need a way to store a bunch of if else if ...
	}

	sif->set_expression(e);
}

void parser::for_p(for_expression_t *sfor)
{
	expression_t *e1;
	expression_t *e2;
	expression_t *e3;

	if (next().token != LPAR)
	{
		error_expected("(");
	}

	m_it++;

	// optional expression
	if (next().token != SEMI)
	{
		e1 = expression_p();
		// f->stmts.push_back(s);

		if (next().token != SEMI)
		{
			error_expected(";");
		}
	}

	m_it++;

	if (next().token != SEMI)
	{
		e2 = expression_p();

		if (next().token != SEMI)
		{
			error_expected(";");
		}
	}

	m_it++;

	if (next().token != RPAR)
	{
		e3 = expression_p();

		if (next().token != RPAR)
		{
			error_expected(")");
		}
	}

	m_it++;

	statement_or_block(sfor);

	sfor->set_expression1(e1);
	sfor->set_expression2(e2);
	sfor->set_expression3(e3);
}

void parser::while_p(while_expression_t *swhile)
{
	expression_t *e;

	if (next().token != LPAR)
	{
		error_expected("(");
	}

	m_it++;

	e = expression_p();

	if (next().token != RPAR)
	{
		error_expected(")");
	}

	m_it++;

	statement_or_block(swhile);

	swhile->set_expression(e);
}

void parser::do_p(do_expression_t *sdo)
{
	expression_t *e;

	statement_or_block(sdo);

	if (next().token != WHILE)
	{
		error_expected("while");
	}

	m_it++;

	if (next().token != LPAR)
	{
		error_expected("(");
	}

	m_it++;

	e = expression_p();

	if (next().token != RPAR)
	{
		error_expected(")");
	}

	m_it++;

	sdo->set_expression(e);
}

// ternary
expression_t *parser::expression_p()
{
	ternary_expression_t *boolean = (ternary_expression_t *)dpipe();
	binary_expression_t *left;
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == QUEST)
	{
		lexeme op = next();
		m_it++;
		left = dpipe();

		if (next().token != COLON)
		{
			error_expected(":");
		}

		m_it++;

		right = dpipe();
		temp = boolean;

		boolean = new ternary_expression_t;
		boolean->boolean = temp;
		boolean->left = left;
		boolean->right = right;
		boolean->op = op;
	}

	return boolean;
}

binary_expression_t *parser::dpipe()
{
	binary_expression_t *left = damp();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == DPIPE)
	{
		lexeme op = next();
		m_it++;
		right = damp();
		temp = left;
		left = new binary_expression_t;

		left->left = temp;
		left->right = right;
		left->op = op;
	}

	return left;
}

binary_expression_t *parser::damp()
{
	binary_expression_t *left = pipe();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == DAMP)
	{
		lexeme op = next();
		m_it++;
		right = pipe();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::pipe()
{
	binary_expression_t *left = amp();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == PIPE)
	{
		lexeme op = next();
		m_it++;
		right = amp();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::amp()
{
	binary_expression_t *left = eq();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == AMP)
	{
		lexeme op = next();
		m_it++;
		right = eq();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::eq()
{
	binary_expression_t *left = comp();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == EQ || next().token == NE)
	{
		lexeme op = next();
		m_it++;
		right = comp();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::comp()
{
	binary_expression_t *left = sum();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == LT || next().token == LE || next().token == GT || next().token == GE)
	{
		lexeme op = next();
		m_it++;
		right = sum();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::sum()
{
	binary_expression_t *left = product();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == PLUS || next().token == MINUS)
	{
		lexeme op = next();
		m_it++;
		right = product();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

binary_expression_t *parser::product()
{
	binary_expression_t *left = (binary_expression_t *)expression();
	binary_expression_t *right;
	binary_expression_t *temp;

	while (next().token == STAR || next().token == SLASH || next().token == MOD)
	{
		lexeme op = next();
		m_it++;
		right = (binary_expression_t *)expression();
		temp = left;

		left = new binary_expression_t;
		left->left = temp;
		left->op = op;
		left->right = right;
	}

	return left;
}

expression_t *parser::expression()
{
	lexeme op = next();
	expression_t *e;

	switch (op.token)
	{
	case tokentype::STR_LIT:
	case tokentype::INT_LIT:
	case tokentype::CHAR_LIT:
	case tokentype::REAL_LIT:
		e = new expression_t;
		e->op = next();
		m_it++;

		return e;

	case tokentype::IDENT: // function call or lvalue
		m_it++;

		// lvalue
		if (next().token != tokentype::LPAR)
		{
			e = lvalue_expression(op);

			return e;
		}

		m_it++;

		// function call with parameters
		if (next().token != RPAR)
		{
			// function_call_t *fc = new function_call_t;
			// fc->function_name = op;
			// fc->args.push_back(expression_p(f));
			// f->function_calls.push_back(fc);
			e = new function_call_t;
			((function_call_t *)e)->op = op;
			((function_call_t *)e)->args.push_back(expression_p());

			while (next().token == COMMA)
			{
				m_it++;

				((function_call_t *)e)->args.push_back(expression_p());
			}
		}
		else
		{
			e = new function_call_t;
			((function_call_t *)e)->op = op;
		}

		m_it++;

		return e;

	case tokentype::AMP:
	case tokentype::STAR:
	case tokentype::PLUS:
	case tokentype::MINUS:
	case tokentype::TILDE:
	case tokentype::BANG:
		e = new unary_expression_t;
		e->op = op;

		m_it++;
		((unary_expression_t *)e)->left = expression_p();
		return e;

	case LPAR: // could be a cast
		m_it++;
		if (next().token == TYPE)
		{
			// better be a cast
			e = new unary_expression_t;
			e->op = next();

			m_it++;

			if (next().token != RPAR)
			{
				error_expected(")");
			}

			m_it++;

			((unary_expression_t *)e)->left = expression_p();
			return e;
		}
		else
		{
			e = expression_p();

			if (next().token != RPAR)
			{
				error_expected(")");
			}

			m_it++;

			return e;
		}

		break;
	case INCR:
		e = new unary_expression_t;
		e->op = op;

		m_it++;
		lvalue_expression(op);
		((unary_expression_t *)e)->left = expression_p();

		return e;

	case DECR:
		e = new unary_expression_t;
		e->op = op;

		m_it++;
		lvalue_expression(op);
		((unary_expression_t *)e)->left = expression_p();

		return e;

	default:
		// other cases

		error_expected("identifier (within expression)");
		return nullptr;
	}

	return nullptr;
}

expression_t *parser::lvalue_expression(lexeme &op)
{
	expression_t *e, *a; // = new binary_expression_t;
	e = nullptr;
	a = nullptr;

	// array indexing
	if (next().token == LBRAK)
	{
		a = new array_expression_t;

		m_it++;

		expression_t *temp = expression_p();
		((array_expression_t *)a)->set_expression(temp); // array

		// if (!a->op.lineno)
		a->op = op;

		if (next().token != RBRAK)
		{
			error_expected("identifier");
		}
		m_it++;
	}
	else
	{
		a = new expression_t;
		a->op = op;
	}

	if ((e = lvalue_p()))
	{
		((binary_expression_t *)e)->left = a;

		return e;
	}
	else
	{
		if (!a)
		{
			e = new expression_t;
			e->op = op;

			return e;
		}
		else
		{
			return a;
		}
	}

	return nullptr;
}

expression_t *parser::lvalue_p()
{
	expression_t *e = nullptr;

	switch (next().token)
	{
	case tokentype::ASSIGN:
	case tokentype::STARASSIGN:
	case tokentype::SLASHASSIGN:
	case tokentype::PLUSASSIGN:
	case tokentype::MINUSASSIGN:

		e = new binary_expression_t;
		e->op = next();
		m_it++;

		((binary_expression_t *)e)->right = expression_p();

		break;
	case tokentype::INCR:
	case tokentype::DECR:
		e = new unary_expression_t;
		e->op = next();
		m_it++;

		break;
	default:
		break;
	}

	return e;
}

int parser::type_check()
{
	for (size_t i = 0; i < m_global_variables.size(); i++)
	{
		variable_t *v = m_global_variables[i];
		if (string_to_type(v->type) == VOID)
		{
			std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
			std::cerr << "\tvariables cannot have type void" << std::endl;

			exit(1);
		}

		// check if this variable has already been declared
		for (size_t j = 0; j < i; j++)
		{
			if (v->var == m_global_variables[j]->var)
			{
				std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
				std::cerr << "\tvariable redeclared" << std::endl;

				exit(1);
			}
		}
	}

	for (size_t k = 0; k < m_functions.size(); k++)
	{
		function_t *f = m_functions[k];

		for (size_t i = 0; i < f->local_variables.size(); i++)
		{
			variable_t *v = f->local_variables[i];

			// check if variables have type void
			if (string_to_type(v->type) == VOID)
			{
				std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
				std::cerr << "\t"
						  << "variables cannot have type void" << std::endl;

				exit(1);
			}

			// check if this variable has already been declared
			for (size_t j = 0; j < i; j++)
			{
				if (v->var == f->local_variables[j]->var)
				{
					std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
					std::cerr << "\tvariable redeclared" << std::endl;

					exit(1);
				}
			}

			// check if variable is same as function parameter
			for (size_t j = 0; j < f->params.size(); j++)
			{
				variable_t *par = f->params[j];
				if (par->var == v->var)
				{
					std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
					std::cerr << "\tvariable cannot have the same name as a parameter" << std::endl;

					exit(1);
				}
			}
		}

		// check if function has same parameter name more than once
		for (size_t i = 0; i < f->params.size(); i++)
		{
			variable_t *v = f->params[i];

			for (size_t j = 0; j < i; j++)
			{
				variable_t *prev = f->params[j];
				if (v->var == prev->var)
				{
					std::cerr << "Type checking error in file " << v->lex.infile_name << " line " << v->lex.lineno << " at text " << v->lex.lex << std::endl;
					std::cerr << "\tparameter redeclared" << std::endl;

					exit(1);
				}
			}
		}

		// check if function has been declared with same name
		for (size_t i = 0; i < k; i++)
		{
			function_t *prev = m_functions[i];

			if (f->func == prev->func)
			{
				std::cerr << "Type checking error in file " << f->lex.infile_name << " line " << f->lex.lineno << " at text " << f->func << std::endl;
				std::cerr << "\tfunction with the same name already exists" << std::endl;

				exit(1);
			}
		}

		for (expression_t *e : f->block.stmts)
		{
			e->print(this, f);

#ifdef DEBUG
			std::cout << std::endl;
#endif
		}
	}

	return 0;
}

parser_type parser::has_function(function_call_t *function) const
{
	bool valid = true;

	// first check for program defined functions

	for (function_t *f : m_functions)
	{
		// matching function name
		if (f->func == function->op.lex)
		{
			if (f->params.size() == function->args.size())
			{
				// check individual arg types
				for (size_t i = 0; i < f->params.size(); i++)
				{
					if (string_to_type(f->params[i]->type) != function->args[i]->derived_type || f->params[i]->is_array != function->args[i]->is_array)
					{
						valid = false;
						break;
					}
				}

				if (valid)
				{
					return string_to_type(f->return_type);
				}
			}
		}
	}

	// now check for "coms 440 standard library" functions

	if (function->op.lex == "getchar")
	{
		if (function->args.size() == 0)
		{
			return INT;
		}
	}

	if (function->op.lex == "putchar")
	{
		if (function->args.size() == 1 && function->args[0]->derived_type == INT)
		{
			return INT;
		}
	}

	if (function->op.lex == "getint")
	{
		if (function->args.size() == 0)
		{
			return INT;
		}
	}

	if (function->op.lex == "putint")
	{
		if (function->args.size() == 1 && function->args[0]->derived_type == INT)
		{
			return VOID;
		}
	}

	if (function->op.lex == "getfloat")
	{
		if (function->args.size() == 0)
		{
			return FLOAT;
		}
	}

	if (function->op.lex == "putfloat")
	{
		if (function->args.size() == 1 && function->args[0]->derived_type == FLOAT)
		{
			return FLOAT;
		}
	}

	if (function->op.lex == "putstring")
	{
		if (function->args.size() == 1 && function->args[0]->derived_type == CHAR && function->args[0]->is_array)
		{
			return VOID;
		}
	}

	return parser_type::ERROR;
}

variable_t *parser::has_global_variable(std::string &variable) const
{
	for (variable_t *v : m_global_variables)
	{
		if (v->var == variable)
		{
			return v;
		}
	}

	return nullptr;
}

parser::~parser()
{
	for (function_t *f : m_functions)
	{
		delete f;
	}

	for (variable_t *s : m_global_variables)
	{
		delete s;
	}
}
