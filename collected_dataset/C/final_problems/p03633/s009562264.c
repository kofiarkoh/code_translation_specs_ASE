#include <stdio.h>
 

long long euclidGcd(long long inputNum1, long long inputNum2)
{
	long long minNumGcd = 0;
	long long maxNumGcd = 0;
	long long euclideanAlgorithmGcd = 1;	//ユークリッドの互除法によりGcd(greatest common divisor)最大公約数

	//--
	//数値の大小関係(--> if,else)
	//ユークリッドの互除法により最大公約数を求める。euclideanAlgrithimGcdを0にしないように、while実行されない(--> for(交換))
	//最大公約数を返す(--> return)
	if (inputNum1 > inputNum2)
	{
		maxNumGcd = inputNum1;	
		minNumGcd = inputNum2;
	}
	else
	{
		maxNumGcd = inputNum2;
		minNumGcd = inputNum1;
	}


	while (euclideanAlgorithmGcd != 0)
	{
		//見にくい
		euclideanAlgorithmGcd = maxNumGcd % minNumGcd;	// aaa = bbb % ccc;
		maxNumGcd = minNumGcd;							// bbb = ccc;
		minNumGcd = euclideanAlgorithmGcd;				// ccc = aaa;
	}

	
	euclideanAlgorithmGcd = maxNumGcd;
	return euclideanAlgorithmGcd;
	//--
}

long long euclidLcm(long long inputNum1, long long inputNum2, long long inputNumGcd)
{
	long long euclideanAlgorithmLcm = 0;	

	//--
	//最小公倍数の公式　＝ inputNum1 * inputNum2 /euclideanAlgorithmGcd(最大公約数);(--> 公式
	euclideanAlgorithmLcm = inputNum1 * (inputNum2 / inputNumGcd);
	//--
	return euclideanAlgorithmLcm;

}


int main(void)
{
	int input = 0;
	long long answer = 0;
	long long euclideanAlgorithmGcd = 0;

	long long t[100] = {};

	//数字を受け取る(--> scanf N個入力される予定)
	//数値の受け取り(--> for(scanf))
	//ユークリッドの互除法により、最小公倍数を計算(--> for(euclidGcd,euclidLcm))
	//答えの表示(--> printf)
	//-
	scanf("%d", &input);

	for (int i = 0; i < input;++i)
	{
		scanf("%lld",&t[i]);
	}

	answer = t[0];

	for (int i = 0; i < input - 1; i++) {
		euclideanAlgorithmGcd = euclidGcd(answer, t[i + 1]);
		answer = euclidLcm(answer, t[i + 1], euclideanAlgorithmGcd);
	}
	
	printf("%lld", answer);
	//--

	return 0;

}
