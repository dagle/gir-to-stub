#!/bin/lua5.4

local lgi = require("lgi")
local gmime = lgi.require("GMime", "3.0")
local gtk = lgi.require("Gtk", "3.0")
local GObject = lgi.GObject
require("debugger")

gmime.init()
gtk.init()

local mes = gmime.Message.new()
local win = gtk.Window()

function dump_props(obj)
  print("Dumping properties of ", obj)
  for _, pspec in pairs(obj._class:list_properties()) do
    print(pspec.name, pspec.value_type)
  end
end

-- dump_props(GObject)

print(mes.bind_property)
-- dump(GObject:_resolve(true), 1000)
