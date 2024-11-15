defmodule CallerTest do
  use ExUnit.Case
  doctest Caller

  test "greets the world" do
    assert Caller.hello() == :world
  end
end
