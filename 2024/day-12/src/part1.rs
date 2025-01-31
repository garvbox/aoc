use std::collections::VecDeque;

use glam::IVec2;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Plant {
    position: IVec2,
    letter: char,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Region {
    plants: Vec<Plant>,
    letter: char,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let mut plants: VecDeque<Plant> = input
        .lines()
        .rev()
        .skip_while(|line| line.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            tracing::trace!("Line {row} - {line:?}");
            line.chars()
                .enumerate()
                .map(|(col, ch)| Plant {
                    position: IVec2::new(col as i32, row as i32),
                    letter: ch,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    tracing::debug!("Plants: {plants:?}");

    let mut regions: Vec<Region> = Vec::new();
    let mut current_region: Region = Region {
        letter: plants[0].letter,
        plants: Vec::new(),
    };
    let mut last_plant_pos = IVec2::new(0, 0);

    while let Some(plant) = plants.pop_front() {
        tracing::trace!(
            "Popped Plant {plant:?}, Region: {current_region:?}, last pos: {last_plant_pos:?}"
        );
        // When we jump to a smaller plant position we have looped around to the start,
        // get next region
        if last_plant_pos.x < plant.position.x && last_plant_pos.y < plant.position.y {
            regions.push(current_region);
            current_region = Region {
                letter: plant.letter,
                plants: Vec::new(),
            };
        };

        if plant.letter == current_region.letter && in_region(&current_region, &plant) {
            current_region.plants.push(plant);
        } else {
            tracing::trace!("Pushed back {plant:?}");
            // plants.push_back(plant);
        }
        last_plant_pos = plant.position;
    }

    Ok("".to_string())
}

fn in_region(region: &Region, plant: &Plant) -> bool {
    // TODO: This is a placeholder, only returing single-char regions
    return region.plants.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        assert_eq!("140", process(input)?);
        Ok(())
    }
}
