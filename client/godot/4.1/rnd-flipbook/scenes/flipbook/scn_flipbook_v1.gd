extends Control

signal critical_error

@export var flipbook_asset_URL: String = "fb_000.json"
# Where this particular object is auto loaded in runtime (see projects Autoloads for more details) 
var url: String = CONSTANTS.construct_url_of(flipbook_asset_URL)

@onready var http_request_json: HTTPRequest = $HTTPRequest_json
@onready var http_request_binary: HTTPRequest = $HTTPRequest_binary

var fb_meta: FlipbookMetadataV1 = null
var _current_page: int = 0
var _json_parse: JSON
var language: String


# Called when the node enters the scene tree for the first time.
func _ready():
	print_debug("Attempting to load: " + self.url)
	var _ignore = self.http_request_json.connect("request_completed", _on_json_request_completed)
	_ignore = self.http_request_json.request(self.url)


func load_page(index: int) -> void:
	print_debug("Loading page: %s" % index)
	var img = self.fb_meta.get_image_on_page(index)
	var texture : ImageTexture = ImageTexture.create_from_image(img)
	var texture_rect = ($backdrop as TextureRect)
	texture_rect.texture = texture
	
	var asp : AudioStreamPlayer = ($AudioStreamPlayer as AudioStreamPlayer)
	if asp.playing:
		print_debug("AudioStreamPlayer was playing, stopping")
		asp.stop()
		asp.stream = null
	var audio_stream = self.fb_meta.get_audio_on_page(index, self.language)
	if audio_stream != null:
		asp.stream = audio_stream
		asp.play(0)
 

func _on_next_page_pressed() -> void:
	print("Call to next page received")
	self._current_page += 1
	if self._current_page == self.fb_meta.get_page_count():
		self._current_page = 0
	self.load_page(self._current_page)


func _on_json_request_completed(result: int, response_code: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
	print_debug("on_json_request_completed called!")
	if result != OK:
		push_error("json request to `%s` raised an error" % self.url)
		# TODO probably signal something (?) some form of error control of sorts?
		emit_signal("critical_error", "json HTTPrequest failed")
		return
	
	if response_code != 200:
		push_error("response code received '%s'" % response_code)
		push_error("json request to `%s` response code != 200" % self.url)
		emit_signal("critical_error", "json HTTPrequest response code: %s" % response_code )
		return

	var json_request: JSON = JSON.new()
	var parse_error : Error = json_request.parse(body.get_string_from_utf8())
	if parse_error != OK:
		push_error("Error parsing JSON: %s" % json_request.get_error_message() )
		push_error("Error parsing JSON: %s" % json_request.get_error_line() )
		emit_signal("critical_error", "Error parsing metadata JSON")
		return
	
	self._json_parse = json_request
	
	# Let's chain the next request
	var _ignore = self.http_request_binary.connect("request_completed", _on_binary_request_completed)
	_ignore = self.http_request_binary.request(CONSTANTS.construct_url_of("%s" % (self._json_parse.data as Dictionary)["binary_package_url"]))

# Called after loading the binary blob contained in the a
func _on_binary_request_completed(result: int, response_code: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
	print_debug("on_binary_request_completed called!")
	if result != OK:
		push_error("binary request raised an error")
		emit_signal("critical_error", "json HTTPrequest failed")
		return
	
	if response_code != 200:
		push_error("request request to  response code != 200")
		emit_signal("critical_error", "json HTTPrequest response code: %s" % response_code )
		return

	self.fb_meta = FlipbookMetadataV1.new(self._json_parse.data as Dictionary, body)
	self.language = self.fb_meta.get_default_language()
	print_debug("Request for binaries succeeded")
	self.load_page(0)
