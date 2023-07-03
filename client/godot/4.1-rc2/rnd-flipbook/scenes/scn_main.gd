extends Control

@onready var get_all_v1: HTTPRequest = $HTTPRequest_get_all_v1
@onready var lbl_answer: Label = $CenterContainer/VBoxContainer/lbl_answer
@onready var lbl_error: Label = $CenterContainer/VBoxContainer/lbl_error
@onready var lbl_server_name : Label = $CenterContainer/VBoxContainer/lbl_server_name

# Called when the node enters the scene tree for the first time.
func _ready():
	# Only for visibility reasons
	self.lbl_server_name.text = CONSTANTS.SERVER
	print_debug("Requesting: %s" % CONSTANTS.url_get_all_v1())
	self.get_all_v1.connect("request_completed", _on_http_request_get_all_v_1_request_completed)
	var error: int =  self.get_all_v1.request(CONSTANTS.url_get_all_v1())
	if error != OK:
		self.lbl_error.visible = true
		self.lbl_error.text = "Error requesting all V1"
		push_error("Error requesting get_all_v1")


func _on_http_request_get_all_v_1_request_completed(result: int, _response_code: int, _headers: PackedStringArray, _body: PackedByteArray):
	print_debug("Answer received %s" % result)
	self.lbl_answer.visible = true
	self.lbl_answer.text = "Received"

