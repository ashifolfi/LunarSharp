-- TODO: finish filling this out
local udtypes = {
	"mobj_t", "player_t", "sector_t", "line_t"
}

_G.assert_type = function(value, typeval)
	local type_userdata = false
	for _,v in ipairs(udtypes) do
		if typeval == v then
			type_userdata = true
			break
		end
	end
	
	if not(type_userdata) then
		if type(value) == typeval then
			return
		else
		
			error("[ExceptionIncorrectType] : expected type '"..typeval.."' but got '"..type(value).."'!")
		end
	end
	
	if userdatatype(value) == typeval then
		return
	else
		error("[ExceptionIncorrectType] : expected type '"..typeval.."' but got '"..userdatatype(value).."'!")
	end
end