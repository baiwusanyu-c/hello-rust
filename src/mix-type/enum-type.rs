// 枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。
// 和 ts 枚举不同，rust 枚举可以是不同类型
fn enumDemo(){
    enum PokerSuit {
        Clubs(char),
        Spades(char),
        Diamonds(u8),// (u8) 类型枚举
        Hearts(u8),
      }
      // 枚举通过 :: 来访问，可以赋值、
      let c1 = PokerCard::Spades(5);
      let c2 = PokerCard::Diamonds(13);
}