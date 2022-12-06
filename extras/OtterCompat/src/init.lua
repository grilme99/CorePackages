local Flipper = require(script.Parent.Flipper)

local function createGroupMotor(initialValues)
	return Flipper.GroupMotor.new(initialValues, true)
end

local function createSingleMotor(initialValue)
	return Flipper.SingleMotor.new(initialValue, true)
end

return {
	createGroupMotor = createGroupMotor,
	createSingleMotor = createSingleMotor,
	spring = Flipper.Spring.new,
	instant = Flipper.Instant.new,
}
