require 'spec_helper'

describe HelixRuntime do

  module TYPES
    UNTESTED = {}

    CASES = {
      T_NONE:     UNTESTED,
      T_NIL:      nil,
      T_OBJECT:   Object.new,
      T_CLASS:    Class.new,
      T_ICLASS:   UNTESTED,
      T_MODULE:   Module.new,
      T_FLOAT:    1.5,
      T_STRING:   "hello",
      T_REGEXP:   /hello/,
      T_ARRAY:    [],
      T_HASH:     {},
      T_STRUCT:   Struct.new(:hello).new,
      T_BIGNUM:   2 ** 65,
      T_FILE:     File.open(__FILE__),
      T_FIXNUM:   2,
      T_TRUE:     true,
      T_FALSE:    false,
      T_DATA:     UNTESTED,
      T_MATCH:    "hello".match(/hello/),
      T_SYMBOL:   :hello,
      T_RATIONAL: Rational(1, 2),
      T_COMPLEX:  Complex(1, 2),
      T_UNDEF:    UNTESTED,
      T_NODE:     UNTESTED,
      T_ZOMBIE:   UNTESTED,
      T_MASK:     UNTESTED
    }
  end

  it 'has a version number' do
    expect(HelixRuntime::VERSION).not_to be nil
  end

  it 'exports the Qtrue macro' do
    expect(Dummy::Qtrue).to equal(true)
  end

  it 'exports the Qfalse macro' do
    expect(Dummy::Qfalse).to equal(false)
  end

  it 'exports the Qnil macro' do
    expect(Dummy::Qnil).to equal(nil)
  end

  it 'exports the RSTRING_LEN macro' do
    expect(Dummy.RSTRING_LEN('hello')).to equal(5)
    expect { Dummy.RSTRING_LEN(1) }.to segv
  end

  it 'exports the RSTRING_PTR macro' do
    expect(Dummy.RSTRING_PTR('hello')).to_not eq(Dummy::RSTRING_PTR('hello'))
    expect(Dummy.RSTRING_PTR('hello'.freeze)).to eq(Dummy::RSTRING_PTR('hello'.freeze))
    expect { Dummy.RSTRING_PTR(1) }.to segv
  end

  it 'exports the RARRAY_LEN macro' do
    expect(Dummy.RARRAY_LEN([1,2,3,4,5])).to equal(5)
    expect { Dummy.RARRAY_LEN(1) }.to segv
  end

  it 'exports the RARRAY_PTR macro' do
    arr = [1,2,3,4,5]
    expect(Dummy.RARRAY_PTR([1,2,3,4,5])).to_not eq(Dummy::RARRAY_PTR([1,2,3,4,5]))
    expect(Dummy.RARRAY_PTR(arr)).to eq(Dummy::RARRAY_PTR(arr))
    expect { Dummy.RARRAY_PTR(1) }.to segv
  end

  describe 'coercions' do
    it "(INT2FIX)" do
      expect(Dummy.INT2FIX(10)).to eq(10)
    end

    it "(FIX2INT)" do
      expect(Dummy.FIX2INT(10)).to eq(10)
    end
  end

  describe "exports T_* constants:" do
    TYPES::CASES.each do |type_name, obj|
      next if obj == TYPES::UNTESTED
      type = Dummy.const_get(type_name)

      describe "#{obj.class} is #{type_name}" do
        it "(RB_TYPE_P)" do
          expect(Dummy.RB_TYPE_P(obj, type)).to be(true)
          expect(Dummy.RB_TYPE_P(obj, Dummy::T_NONE)).to be(false)
        end

        it "(TYPE)" do
          expect(Dummy.TYPE(obj)).to be(type)
        end
      end
    end
  end

  # it 'exports the RB_TYPE_P macro and T_*' do
  #   expect(Dummy.RB_TYPE_P("hello", Dummy::T_STRING)).to be(true)
  #   expect(Dummy.RB_TYPE_P({}, Dummy::T_HASH)).to be(true)
  #   expect(Dummy.RB_TYPE_P([], Dummy::T_OBJECT)).to be(false)
  # end

  # it 'exports the TYPE macro' do
  #   expect(Dummy.TYPE("hello")).to eq(Dummy::T_STRING)
  #   expect(Dummy.TYPE({})).to eq(Dummy::T_HASH)
  #   expect(Dummy.TYPE([])).to_not eq(Dummy::T_OBJECT)
  # end

  describe "helix_rb_utf8_str_new" do
    it "allocates a new string" do
      str1 = "hello world"
      str2 = Dummy.STR2STR(str1, 5)

      expect(str2).to eq("hello")

      str1[0...5] = "goodbye"

      expect(str1).to eq("goodbye world")
      expect(str2).to eq("hello")

      str2 << " world!"

      expect(str1).to eq("goodbye world")
      expect(str2).to eq("hello world!")
    end
  end

  describe "Data_{Wrap,Get,Set}_Struct" do
    it "can allocate then change the data" do
      wrapper = Dummy::Wrapper.new

      expect(Dummy.get_data(wrapper)).to eq(0)

      ptr = Dummy.get_data_ptr(wrapper)

      expect(Dummy.set_data(wrapper, 1)).to eq(1)

      expect(Dummy.get_data(wrapper)).to eq(1)
      expect(Dummy.get_data_ptr(wrapper)).to eq(ptr)
    end

    it "can allocate then replace the data" do
      wrapper = Dummy::Wrapper.new

      expect(Dummy.get_data(wrapper)).to eq(0)

      ptr = Dummy.get_data_ptr(wrapper)

      expect(Dummy.replace_data(wrapper, 1)).to eq(1)

      expect(Dummy.get_data(wrapper)).to eq(1)
      expect(Dummy.get_data_ptr(wrapper)).not_to eq(ptr)
    end
  end
end

