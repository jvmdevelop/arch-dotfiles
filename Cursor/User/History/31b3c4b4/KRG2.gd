extends VehicleBody3D

# ==== ПАРАМЕТРЫ ДВИГАТЕЛЯ ====
@export var max_rpm: float = 8000.0
@export var idle_rpm: float = 900.0
@export var max_torque: float = 450.0  # Нм
@export var engine_brake: float = 0.15

# ==== ТРАНСМИССИЯ ====
@export var gear_ratios := [-3.2, 0, 3.1, 2.2, 1.6, 1.3, 1.0, 0.85]
@export var final_drive: float = 3.42
var current_gear: int = 1
var rpm: float = 1000.0
var clutch: float = 1.0

@export var steer_speed: float = 4.0
@export var max_steer_angle: float = 0.5
@export var brake_power: float = 4000.0
@export var handbrake_power: float = 6000.0

# ==== АЭРОДИНАМИКА ====
@export var downforce_strength: float = 50.0  # Сила прижима к земле (увеличивается со скоростью)
@export var max_speed_for_steering: float = 100.0  # Скорость (км/ч), при которой руль полностью теряет эффективность
@export var min_steer_multiplier: float = 0.3  # Минимальный множитель поворачиваемости на высокой скорости (0.3 = 30%)

var throttle_input := 0.0
var brake_input := 0.0
var steer_input := 0.0

func _physics_process(delta):
	throttle_input = Input.get_action_strength("accelerate")
	brake_input = Input.get_action_strength("brake")
	steer_input = Input.get_action_strength("steer_left") - Input.get_action_strength("steer_right")

	# Вычисляем текущую скорость (м/с -> км/ч)
	var speed_ms = linear_velocity.length()
	var speed_kmh = speed_ms * 3.6

	# Прижимная сила: увеличивается пропорционально квадрату скорости (как в реальной аэродинамике)
	var downforce = downforce_strength * speed_ms * speed_ms
	apply_central_force(Vector3.DOWN * downforce)

	# Поворачиваемость: уменьшается с увеличением скорости
	var speed_factor = clamp(speed_kmh / max_speed_for_steering, 0.0, 1.0)
	var steer_multiplier = lerp(1.0, min_steer_multiplier, speed_factor)
	var effective_steer_angle = max_steer_angle * steer_multiplier

	steering = lerp(steering, steer_input * effective_steer_angle, steer_speed * delta)

	var wheel_rpm = get_wheel_rpm()
	rpm = max(idle_rpm, abs(wheel_rpm * gear_ratios[current_gear] * final_drive))

	if rpm > max_rpm:
		rpm = max_rpm

	var torque = torque_curve(rpm) * throttle_input

	var drive_force = torque * gear_ratios[current_gear] * final_drive * clutch
	engine_force = -drive_force

	brake = brake_input * brake_power

	# Handbrake should only lock the rear wheels while held;
	# make sure we reset wheel.brake when it is released.
	for wheel in get_children():
		if wheel is VehicleWheel3D and wheel.name.contains("R"):
			wheel.brake = 0.0
			if Input.is_action_pressed("handbrake"):
				wheel.brake = handbrake_power

	if Input.is_action_just_pressed("gear_up"):
		current_gear = clamp(current_gear + 1, 0, gear_ratios.size() - 1)

	if Input.is_action_just_pressed("gear_down"):
		current_gear = clamp(current_gear - 1, 0, gear_ratios.size() - 1)


func torque_curve(current_rpm: float) -> float:
	var normalized = current_rpm / max_rpm
	
	var torque_factor = 1.0 - pow(normalized - 0.6, 2) * 2.5
	torque_factor = clamp(torque_factor, 0.2, 1.0)

	return max_torque * torque_factor


func get_wheel_rpm() -> float:
	var sum := 0.0
	var count := 0
	for wheel in get_children():
		if wheel is VehicleWheel3D and wheel.use_as_traction:
			sum += wheel.get_rpm()
			count += 1
	if count == 0:
		return 0
	return sum / count
