use chocomint;

#[test]
fn dev() {
	let token = "_|WARNING:-DO-NOT-SHARE-THIS.--Sharing-this-will-allow-someone-to-log-in-as-you-and-to-steal-your-ROBUX-and-items.|_CBA385B87AC4BB84C0ED21C20C1D741D66809B3020C1DEC35A4C648065BF633619788EC1F5387A62B7471E55FC7A8F6CD7C94A29DEF321BCCF7B462F77185A6ACE6A6AF1B3A5A65D8CC6EE790A46A427053054DB9799CA994E1AD5F60D7499F159049626308058AE1CF5EC41B923AFD334E209BF94BDBAEE8EB62BF9D9AF22BB5ABFB564EDA92FCC5DA068A7D3048F3C9622631CD6E0223FD05A5A5A6A69B44AE21FFB243105BE5F59D02166116BACAA1DF8C428A71EFE986470A2380AFEDCC5D5014F08B14A9CFC9EC252300027DEA3805B7880E48BA21B50454E9D340D6547F41B029224B07682E2F105FA0F0C524560ABDCC284FB3EDA3FF0F50C6A27F359FE80D861209D8BA4CD227D5456D120024F051650B4203A5141DF139D315C4EB219FA1ADB25B1FFE0394BFBFDBE26A2D3E684DCCC";
	let _client = chocomint::client::new(&token)
		.connect();
}