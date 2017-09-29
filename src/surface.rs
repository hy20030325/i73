use chunk::storage::Target;
use biome::Biome;

const SEA_COORD:  u32 = 63;
const BEACH_LOW:  u32 = SEA_COORD - 3;
const BEACH_HIGH: u32 = SEA_COORD + 2;

pub struct Surface<B> where B: Target {
	pub top:  Option<B>,
	pub fill: B
}

enum Beach {
	Sand,
	Gravel,
	Biome
}

impl Beach {
	fn surface<B>(&self, biome: &Biome) -> Surface<B> where B: Target {
		/*match *self {
			Beach::Sand   => Surface { top: Some(Block::Sand), fill: Block::Sand },
			Beach::Gravel => Surface { top: None, fill: Block::Gravel },
			Beach::Biome  => biome.surface()
		}*/
		unimplemented!()
	}
}

struct Stack {
	depth: i32,
	beach: Beach,
	biome: Biome
}

impl Stack {
	fn surface<B>(&self, y: u32, last: &Surface<B>) -> Surface<B> where B: Target {
		let mut surface = if self.depth <= 0 {
			//Surface { top: None, fill: Block::Stone }
			unimplemented!()
		} else if y >= BEACH_LOW && y <= BEACH_HIGH {
			self.beach.surface(&self.biome)
		} else {
			//Surface { top: self.biome.surface().top, fill: last.fill }
			unimplemented!()
		};
		
		if y < SEA_COORD {
			unimplemented!()
			//surface.top = Some(surface.fill);
		}
		
		surface
	}
}