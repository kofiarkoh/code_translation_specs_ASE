// AtCoder キーエンス プログラミング コンテスト 2020
// A - Painting
// 2020.1.20 bal4u

#include <stdio.h>

int main()
{
	int H, W, N, t;
	
	scanf("%d%d%d", &H, &W, &N);
	if (H > W) t = H, H = W, W = t;
	printf("%d\n", (N-1)/W+1);
	return 0;
}
