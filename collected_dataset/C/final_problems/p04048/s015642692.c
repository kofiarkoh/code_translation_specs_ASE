#include <stdio.h>

int triangle[1010][1010];

int main(void)
{
	int i, N, X, b, c, len, dire_flag;
	scanf("%d%d", &N, &X);

	//b=>c方向の壁1
	//a=>b方向の壁2
	//c=>a方向の壁3
	for (i = 0; i <= N; i++){
		triangle[0][i] = 3;
		triangle[i][0] = 2;
		triangle[N - i][i] = 1;
	}

	//終了条件と被らないように一つ進める
	dire_flag = 1;
	b = X - 1;
	c = 1;
	len = 1;

	while (!(b == X && c == 0)) {
		switch (dire_flag){
			case 1:
			if (triangle[b][c] == 3){
				dire_flag = 2;
				b++;
			} else if (triangle[b][c] == 2) {
				dire_flag = 3;
				c--;
			} else {
				triangle[b][c] = 1;
				b--;
				c++;
			}
			break;

			case 2:
			if (triangle[b][c] == 1){
				dire_flag = 3;
				c--;
			} else if (triangle[b][c] == 3) {
				dire_flag = 1;
				b--;
				c++;
			} else {
				triangle[b][c] = 2;
				b++;

			}
			break;

			case 3:
			if (triangle[b][c] == 2){
				dire_flag = 1;
				b--;
				c++;
			} else if (triangle[b][c] == 1) {
				dire_flag = 2;
				b++;
			} else {
				triangle[b][c] = 3;
				c--;
			}
			break;
		}
		len++;
	}

	printf("%d\n", len);

	return 0;
}