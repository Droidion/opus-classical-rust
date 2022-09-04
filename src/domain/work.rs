pub struct Work {
    pub id: i32,
    pub title: String,
    pub year_start: Option<i32>, // Year when composer started the work, if known. Can be used without YearFinish if the work was finished in a single year.
    pub year_finish: Option<i32>, // Year when composer finished the work, if known. Can be used without YearStart if the work was finished in a single year.
    pub average_minutes: Option<i32>, // Approximate length of the work in minutes.
    pub catalogue_name: Option<String>, // Name of the catalogue of composer's works, like "BWV" for Bach or "Op." for Beethoven.
    pub catalogue_number: Option<i32>, // Catalogue number of the work, like 123 for Op. 123
    pub catalogue_postfix: Option<String>, // Postfix for the number of the work in the catalogue, like b in Op. 123b
    pub key: Option<String>, // e.g. C# minor
    pub no: Option<i32>, // Work number in some sequence, like 9 in Symphony No. 9
    pub nickname: Option<String>, // e.g. Great in Beethoven's Symphony No. 9 Great
}