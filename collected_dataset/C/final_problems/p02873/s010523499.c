#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// 内部定数
#define D_ARRAY_MAX		500005									// 最大配列数

// 内部変数
static FILE *szpFpI;											// 入力
static char sc1Str[D_ARRAY_MAX];								// 文字列
static int si1Array[D_ARRAY_MAX];								// 配列
static int siACnt;												// 配列数

// 内部変数 - テスト用
#ifdef D_TEST
	static int siRes;
	static FILE *szpFpA;
	static int siTNo;
#endif

// １行出力
int
fOutLine(
	char *pcpLine				// <I> １行
)
{
	char lc1Buf[1024];

#ifdef D_TEST
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, pcpLine)) {
		siRes = -1;
	}
#else
	printf("%s", pcpLine);
#endif

	return 0;
}

// 改行カット
// 戻り値：文字数
int
fCutCrLf(
	char *pcpStr				// <I> 文字列
)
{
	int i;

	for (i = 0; pcpStr[i] != '\0'; i++) {
		if (pcpStr[i] == '\n') {
			pcpStr[i] = '\0';
			break;
		}
	}

	return i;
}

// 実行メイン
long long
fMain(
)
{
	int i;

	// 文字列 - 取得
	fgets(&sc1Str[1], sizeof(sc1Str) - 1, szpFpI);
	siACnt = fCutCrLf(&sc1Str[1]) + 1;
	sc1Str[0] = '>';
	sc1Str[siACnt] = '<';

	// 配列 - セット - <<
	for (i = 1; i <= siACnt; i++) {
		if (sc1Str[i - 1] == '<' && sc1Str[i] == '<') {
			si1Array[i] = si1Array[i - 1] + 1;
		}
	}

	// 配列 - セット - >>
	for (i = siACnt; i >= 1; i--) {
		if (sc1Str[i - 1] == '>' && sc1Str[i] == '>') {
			si1Array[i] = si1Array[i + 1] + 1;
		}
	}

	// 配列 - セット - <>
	for (i = 1; i <= siACnt; i++) {
		if (sc1Str[i - 1] == '<' && sc1Str[i] == '>') {
			if (si1Array[i - 1] > si1Array[i + 1]) {
				si1Array[i] = si1Array[i - 1] + 1;
			}
			else {
				si1Array[i] = si1Array[i + 1] + 1;
			}
		}
	}

	// 和 - 取得
	long long llSum = 0;
	for (i = 1; i <= siACnt; i++) {
		llSum += si1Array[i];
	}

	return llSum;
}

// １回実行
int
fOne(
)
{
	long long llRet;
	char lc1Buf[1024];

	// データ - 初期化
	memset(si1Array, 0, sizeof(si1Array));						// 配列

	// 入力 - セット
#ifdef D_TEST
	sprintf(lc1Buf, ".\\Test\\T%d.txt", siTNo);
	szpFpI = fopen(lc1Buf, "r");
	sprintf(lc1Buf, ".\\Test\\A%d.txt", siTNo);
	szpFpA = fopen(lc1Buf, "r");
	siRes = 0;
#else
	szpFpI = stdin;
#endif

	// 実行メイン
	llRet = fMain();

	// 出力
	sprintf(lc1Buf, "%lld\n", llRet);
	fOutLine(lc1Buf);

	// 残データ有無
#ifdef D_TEST
	lc1Buf[0] = '\0';
	fgets(lc1Buf, sizeof(lc1Buf), szpFpA);
	if (strcmp(lc1Buf, "")) {
		siRes = -1;
	}
#endif

	// テストファイルクローズ
#ifdef D_TEST
	fclose(szpFpI);
	fclose(szpFpA);
#endif

	// テスト結果
#ifdef D_TEST
	if (siRes == 0) {
		printf("OK %d\n", siTNo);
	}
	else {
		printf("NG %d\n", siTNo);
	}
#endif

	return 0;
}

// プログラム開始
int
main()
{

#ifdef D_TEST
	int i;
	for (i = D_TEST_SNO; i <= D_TEST_ENO; i++) {
		siTNo = i;
		fOne();
	}
#else
	fOne();
#endif

	return 0;
}

