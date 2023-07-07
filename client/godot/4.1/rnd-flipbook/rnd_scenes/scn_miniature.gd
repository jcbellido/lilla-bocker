extends Control

# Should I deserialize here? 
#   ... or before

var _fb_metadata: FlipbookMetadataV1 = null

func _init(fb_v1: FlipbookMetadataV1):
	self._fb_metadata = fb_v1
	var def_lang: String = self._fb_metadata.get_default_language()
	var title = self._fb_metadata.get_title_sid()
	

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
