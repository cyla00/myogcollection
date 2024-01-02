package types

import "time"

type Pattern struct {
	Id                  string    // json patternId
	Owner_id            string    // json patternOwnerId
	Title               string    // json patternTitle
	Pattern_description string    // json patternDescription
	Gallery_paths       []string  // json patternGallery
	Pattern_path        string    // json patternPath
	Materials           []string  // json patternMaterials
	Tools               []string  // json patternTools
	Category            string    // json patternCategory
	CreatedAt           time.Time // json patternCreationDate
}

// MATERIALS LIST:
// laser cut
// cordura
// ripstop
// mesh fabric
// strech fabric
// foam material
// curv
// tegris
// nylon
// Vinyl Polyester Material
// canvas
// velcro
// elastic cord
// cord
// elastic webbing
// webbing
// quick release buckles
// zipper pulls
// buckles
// cobra buckles
// zipper
// carabiner
// buttons
// g hooks
// other

// TOOLS LIST:
// polyester thread
// polycotton thread
// double sided adhesive
// hot knife cutter
// laser cutter
// sewing clips
// seam ripper
// thread snip
// rotary cutter
// Fabric Scissors
// cutting mat
// Measuring Tape
// ruler
// other

// TAGS LIST:
// backpacks
// bags
// plate carriers
// cummerbunds
// slings
// placards
// wallets
// utility pouches
// med pouches
// mag pouches
// admin pouches
// dump pouches
// comms pouches
// water holders
// helmet covers
// utility sleeves
// danglers
// chest rigs
// pants
// combat shirt
// fatigue shirt
// ballistic protection holders
// other
