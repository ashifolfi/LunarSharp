--#region main C# syntax elements

-- case statements (easier conversion)
rawset(_G, "switch", function(cond, vals)
	if vals[cond] then
		vals[cond]()
	else if vals["default"] then
		vals["default"]()
	end
end)

--#endregion