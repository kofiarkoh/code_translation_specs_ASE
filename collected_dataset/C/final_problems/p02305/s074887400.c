#include <stdio.h>
#include <math.h>



struct Circle{   // 円定義構造体
	float x,y;   // x,y:中心点座標
	float r;   // r:円の半径
};


int main(){
	struct Circle C1;
	struct Circle C2;
	int check;
	int d;
	
	scanf("%f %f %f", &C1.x, &C1.y, &C1.r);
	scanf("%f %f %f", &C2.x, &C2.y, &C2.r);
	



	// 円同士の当たり判定
	int r = C1.r + C2.r;   // ２円の半径の合計
	float r2 = C1.r - C2.r;
	int x = C1.x - C2.x;   // X座標の差
	int y = C1.y - C2.y;   // Y座標の差

 d = sqrt(x*x+y*y);

	if(d>r)check=4;
	else if (d==r)  check=3;
	else if(fabs(r2)<d && d<r)check=2;
	else if(d==fabs(r2)) check=1;
	else check=0;

		printf("%d\n",check);

			return 0;
}


