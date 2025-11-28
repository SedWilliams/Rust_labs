#[derive(Debug)]
enum Lib {
    Lib1,
    Lib2,
    Lib3,
    Lib4,
}

impl Lib {
    fn to_str(self) -> String{
        match &self{
            Lib::Lib1 => String::from("One day, I woke up and realized 
            I had [adjective] powers! I put on my [noun] and went out to [verb] the city. 
            As I was [verb ending in -ing], I saw a [adjective] villain causing trouble. I knew I had to stop them, so I [verb] towards them. We started to [verb] and I used my powers to [verb] them. In the end, 
            I was able to [verb] the villain and save the day! Everyone was so [adjective] 
            and grateful for my help. I felt like a true [adjective] superhero."),
            Lib::Lib2 => String::from("Last summer, my family and I went on a camping trip in the 
            [adjective] woods. We packed all of our [noun] and headed out early in the [noun]. 
            When we arrived at the campsite, we started setting up our [noun] and [verb ending
            in -ing] around. As we were [verb ending in -ing], we saw a [adjective] bear in the 
            distance! We quickly gathered all of our [noun] and went to the [noun] to cook dinner. 
            We made [adjective] hot dogs and [adjective] s'mores. After dinner, we sat around the 
            [noun] and told [adjective] stories. Finally, we went to sleep in our [adjective] tent, 
            feeling tired but [adjective]."),
            Lib::Lib3 => String::from("One morning, I woke up feeling very [adjective]. I knew it was going
            to be a [adjective] day. I decided to start my day by [verb ending in -ing] to my 
            favorite [noun], where I [verb] for a while. After that, I decided to [verb] with my 
            best friend, who is always [adjective]. We had so much fun [verb ending in -ing] and
            laughing together. Later, I went to a [adjective] restaurant and ordered a [adjective]
            meal. It was so delicious! Finally, I ended my day by [verb ending in -ing] in my 
            [adjective] bed, feeling grateful for such a [adjective] day."),
            Lib::Lib4 => String::from("One dark and stormy night, my friends and I decided to explore a 
            [adjective] haunted house. As we entered the house, we heard [adjective] noises and saw 
            [adjective] shadows moving about. Suddenly, a [adjective] ghost appeared in front of us 
            and we all [verb ending in -ed] in fear. The ghost led us through the [adjective] house, 
            showing us all of its [adjective] rooms. In one room, we saw a [adjective] coffin and we 
            all [verb ending in -ed] again. As we continued through the house, we found a [adjective]
            book that contained a [adjective] spell. We recited the spell and the ghost disappeared,
            revealing a hidden treasure. We quickly grabbed the treasure and ran out of the haunted
            house as fast as we could, feeling both scared and [adjective].")
        }
    }
}

fn main() {
    Lib::to_str(Lib::Lib4);
    println!("{:?}", Lib::Lib4);
}
