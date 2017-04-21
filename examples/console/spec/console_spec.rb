require "spec_helper"
require "colorize"

describe "Console" do
  let(:console) { Console.new }

  it "can log a string" do
    expect { console.log("hello") }.to println("hello")
  end

  it "can inspect itself" do
    expect { console.inspect }.to print(/Console { .+ }\n\z/)
  end

  it "can call its own methods" do
    expect { console.hello }.to println("hello")
  end

  it "can take multiple arguments" do
    expect { console.loglog("hello", "world") }.to println("hello world")
  end

  it "can take a boolean" do
    expect { console.log_if("hello", true) }.to println("hello")
    expect { console.log_if("world", false) }.to_not print
  end

  it "can return a string" do
    expect(console.colorize("hello")).to eq("hello".colorize(:red))
  end

  it "can return a boolean" do
    expect(console.is_red("hello")).to eq(false)
    expect(console.is_red("hello".colorize(:red))).to eq(true)
  end

  it "can handle panics" do
    expect { console.freak_out }.to raise_error(RuntimeError, "Aaaaahhhhh!!!!!")
    # Do it twice to make sure we cleaned up correctly the first time
    expect { console.freak_out }.to raise_error(RuntimeError, "Aaaaahhhhh!!!!!")
  end

  it "can handle invalid arguments" do
    expect { console.log(123) }.to raise_error(TypeError, "No implicit conversion of 123 into String")
  end

  it "can handle calls back to Ruby" do
    expect(console.call_ruby).to eq("\"Object\", true, true")
  end

  it "can handle invalid calls back to Ruby" do
    # NOTE: This doesn't verify that Rust unwound correctly
    expect {
      console.behave_badly
    }.to raise_error(NameError, "undefined method `does_not_exist' for Object:Class");
  end
end
