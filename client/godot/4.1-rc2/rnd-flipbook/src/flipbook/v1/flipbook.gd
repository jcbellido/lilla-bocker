extends Object

class_name FlipBook_V1

var _from_remote: Dictionary
var _bin: PackedByteArray
var img_miniature: Image

func _init(from_remote: Dictionary, bin: PackedByteArray):
	var version: int = from_remote["version"]
	if version != 1:
		push_error("Only version 1 is supported at the moment")
	self._from_remote = from_remote
	self._bin = bin
	self._construct_miniature()


# Deserializes the base64 embedded texture into a miniature in the object
func _construct_miniature() -> void:
	var t_from_base64 : PackedByteArray = Marshalls.base64_to_raw(self._from_remote["miniature"])
	self.img_miniature = Image.new()
	if self.img_miniature.load_jpg_from_buffer(t_from_base64) != OK:
		push_error("Error loading miniature, is it a JPG?")
	else:
		print_debug("Miniature image created")

func get_image_on_page(index: int) -> Image:
	var fip: FilePositionInPackage = self._image_fip(index)
	if fip == null:
		return null
	
	var pba: PackedByteArray = self._bin.slice(fip.start, fip.start + fip.length - 1)
	var img: Image = Image.new()
	match (fip.format):
		"jpg":
			if img.load_jpg_from_buffer(pba) != OK:
				push_error("Error decoding JPG for image `%s`", index)
				return null
		_: 
			push_error("Image format %s not supported", fip.format)
			return null
	
	return img

func _image_fip(index: int) -> FilePositionInPackage:
	if index < 0 || index >= self.get_page_count():
		push_error("Image index requested out of range")
		return null
	return FilePositionInPackage.new(self._from_remote["images_in_pages"][index])

func _audio_fip(sid: String) -> FilePositionInPackage:
	return FilePositionInPackage.new(self._from_remote["audio"][sid])

func get_audio_on_page(page: int, language: String) -> AudioStreamMP3:
	var sid = "PAGE_%s_%s" % [page, language]
	print_debug("Looking for audio under SID: `%s`" % sid)
	return self._internal_get_audio_on_page(sid)

func _internal_get_audio_on_page(sid: String) -> AudioStreamMP3:
	var fip : FilePositionInPackage = self._audio_fip(sid)
	if fip == null:
		return null
	var pba: PackedByteArray = self._bin.slice(fip.start, fip.start + fip.length - 1)
	var mp3_audio: AudioStreamMP3 = AudioStreamMP3.new()
	mp3_audio.data = pba
	return mp3_audio

## Root level properties 
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

func get_languages() -> Array:
	return self._from_remote["languages"]


