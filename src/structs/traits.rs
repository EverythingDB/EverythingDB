trait HasTitle{
    fn title(&self) -> String{
        self.title
    }
}
trait HasID{
    fn id(&self) -> i64{
        self.id
    }
}