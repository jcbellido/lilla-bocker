extends Object

class_name FlipbookMetadataV1

var _from_remote: Dictionary
var _bin: PoolByteArray
var img_miniature: Image

func _init(from_remote: Dictionary, bin: PoolByteArray):
	self._from_remote = from_remote
	self._bin = bin
	self._construct_miniature()

# Deserializes the base64 embedded texture into a miniature in the object
func _construct_miniature() -> void:
	var t_from_base64 : PoolByteArray = Marshalls.base64_to_raw(self._from_remote["miniature"])
	self.img_miniature = Image.new()
	var _ignore = self.img_miniature.load_jpg_from_buffer(t_from_base64)
	print_debug("Miniature image created")

func get_image_on_page(index: int) -> Image:
	if index < 0 || index >= self.get_page_count():
		push_error("Index requested out of range")
		return null
	var fip: Dictionary = self._from_remote["images_in_pages"][index]
	var pba: PoolByteArray = self._bin.subarray(fip["start"], fip["start"] + fip["length"] - 1)
	var img: Image = Image.new()
	var _ignore = img.load_jpg_from_buffer(pba)
	return img

func get_default_audio(page:int) -> AudioStreamOGGVorbis:
	var sid = "PAGE_%s_%s" % [page, self.get_default_language()]
	print_debug("Looking for audio under SID: `%s`" % sid)
	return self.get_audio(sid)

func get_audio(sid: String) -> AudioStreamOGGVorbis:
	var audio : Dictionary = self._from_remote["audio"]
	if !audio.has(sid):
		print_debug("Can't find audio under SID: `%s`" % sid)
		return null
	var fip: Dictionary = audio[sid]
	var pba: PoolByteArray = self._bin.subarray(fip["start"], fip["start"] + fip["length"] - 1)
	var ogg_audio : AudioStreamOGGVorbis = AudioStreamOGGVorbis.new()
	ogg_audio.data = pba
	return ogg_audio

func get_binary_package_url() -> String:
	return self.from_remote["binary_package_url"]

func get_page_count() -> int:
	return self._from_remote["images_in_pages"].size()

func get_default_language() -> String:
	return self._from_remote["default_language"]

func get_text(sid: String) -> String:
	return self._from_remote["texts"][sid]

func get_title_sid() -> String:
	return "TITLE_%s" % self.get_default_language()

func get_summary_sid() -> String:
	return "SUMMARY_%s" % self.get_default_language() 
