#include <ruby.h>
#include <ruby/intern.h>
#include <stdbool.h>
#include <helix_runtime.h>

VALUE HELIX_Qtrue = Qtrue;
VALUE HELIX_Qfalse = Qfalse;
VALUE HELIX_Qnil = Qnil;

long HELIX_RSTRING_LEN(VALUE string) {
  return RSTRING_LEN(string);
}

const char* HELIX_RSTRING_PTR(VALUE string) {
  return RSTRING_PTR(string);
}

long HELIX_RARRAY_LEN(VALUE array) {
  return RARRAY_LEN(array);
}

void* HELIX_RARRAY_PTR(VALUE array) {
  return RARRAY_PTR(array);
}

bool HELIX_RB_TYPE_P(VALUE v, int type) {
  return RB_TYPE_P(v, type);
}

VALUE HELIX_INT2FIX(int c_int) {
  return INT2FIX(c_int);
}

VALUE HELIX_FIX2INT(VALUE v) {
  return FIX2INT(v);
}

VALUE HELIX_rb_utf8_str_new(const char* str, long len) {
  return rb_utf8_str_new(str, len);
}

VALUE HELIX_Data_Wrap_Struct(VALUE klass, HELIX_RUBY_DATA_FUNC mark, HELIX_RUBY_DATA_FUNC free, void* data) {
  return Data_Wrap_Struct(klass, mark, free, data);
}

void* HELIX_Data_Get_Struct_Value(VALUE obj) {
  void* data;
  Data_Get_Struct(obj, void*, data);
  return data;
}

void HELIX_Data_Set_Struct_Value(VALUE obj, void* data) {
  DATA_PTR(obj) = data;
}

// void HELIX_rb_define_alloc_func(VALUE klass, HELIX_rb_alloc_func_t func) {
//   rb_define_alloc_func(klass, func);
// }

int HELIX_TYPE(VALUE v) {
  return TYPE(v);
}

int HELIX_T_NONE = T_NONE;
int HELIX_T_NIL = T_NIL;
int HELIX_T_OBJECT = T_OBJECT;
int HELIX_T_CLASS = T_CLASS;
int HELIX_T_ICLASS = T_ICLASS;
int HELIX_T_MODULE = T_MODULE;
int HELIX_T_FLOAT = T_FLOAT;
int HELIX_T_STRING = T_STRING;
int HELIX_T_REGEXP = T_REGEXP;
int HELIX_T_ARRAY = T_ARRAY;
int HELIX_T_HASH = T_HASH;
int HELIX_T_STRUCT = T_STRUCT;
int HELIX_T_BIGNUM = T_BIGNUM;
int HELIX_T_FILE = T_FILE;
int HELIX_T_FIXNUM = T_FIXNUM;
int HELIX_T_TRUE = T_TRUE;
int HELIX_T_FALSE = T_FALSE;
int HELIX_T_DATA = T_DATA;
int HELIX_T_MATCH = T_MATCH;
int HELIX_T_SYMBOL = T_SYMBOL;
int HELIX_T_RATIONAL = T_RATIONAL;
int HELIX_T_COMPLEX = T_COMPLEX;
int HELIX_T_UNDEF = T_UNDEF;
int HELIX_T_NODE = T_NODE;
int HELIX_T_ZOMBIE = T_ZOMBIE;
int HELIX_T_MASK = T_MASK;
// int HELIX_T_IMEMO = T_IMEMO;

void Init_native() {}
