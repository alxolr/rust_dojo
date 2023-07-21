use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceThreshold {
    first_place_min_score: u32,
    second_place_min_score: u32,
    third_place_min_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    user_id: String,
    score: u32,
    place: Option<u32>,
}

pub fn calculate_leaderboard_places(users: Vec<User>, min_places: PlaceThreshold) -> Vec<User> {
    let mut sorted_user = users;
    let mut users_with_place = vec![];

    sorted_user.sort_by(|a, b| b.score.cmp(&a.score));

    let mut score_thresholds = vec![
        min_places.third_place_min_score,
        min_places.second_place_min_score,
        min_places.first_place_min_score,
    ];
    let mut place: i32 = 0;

    for mut user in sorted_user {
        while score_thresholds.len() > 0 {
            let score_threshold = score_thresholds.pop().unwrap();
            place += 1;

            if user.score >= score_threshold {
                user.place = Some(place as u32);
                users_with_place.push(user.clone());
                break;
            }
        }

        if score_thresholds.len() == 0 {
            user.place = Some((place + 1) as u32);
            place += 1;
            users_with_place.push(user.clone());
        }
    }

    users_with_place
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use vector_assertions::assert_vec_eq;

    use super::*;

    #[test]
    fn it_works() {
        let users = json!([
            {
                "userId": "id1",
                "score": 3
            },
            {
                "userId": "id2",
                "score": 2
            },
            {
                "userId": "id3",
                "score": 1
            }
        ]);

        let score_threshold = json!({
            "firstPlaceMinScore": 100,
            "secondPlaceMinScore": 50,
            "thirdPlaceMinScore": 10
        });

        let expected = json!([
            {
                "userId": "id1",
                "score": 3,
                "place": 4
            },
            {
                "userId": "id2",
                "score": 2,
                "place": 5
            },
            {
                "userId": "id3",
                "score": 1,
                "place": 6
            }
        ]);

        let users: Vec<User> = serde_json::from_value(users).unwrap();
        let score_threshold: PlaceThreshold = serde_json::from_value(score_threshold).unwrap();

        let result_users = calculate_leaderboard_places(users, score_threshold);
        let expected: Vec<User> = serde_json::from_value(expected).unwrap();

        println!("{:?}", result_users);

        assert_vec_eq!(result_users, expected);
    }
}
