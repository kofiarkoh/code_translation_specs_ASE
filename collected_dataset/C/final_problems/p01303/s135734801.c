#include<stdio.h>

int main(){
	int data_num;
	int X, Y, W, H;
	int x, y, cat_num, cat_campus;
	int i,j;

	scanf("%d",&data_num);

	for(i=0; i<data_num; i++){

		scanf("%d %d %d %d", &X, &Y, &W, &H);
		scanf("%d",&cat_num);

		cat_campus=0;
		for(j=0; j<cat_num; j++){
			scanf("%d %d", &x, &y);
			if(X <= x && x <= W+X && Y <= y && y <= H+Y)
				cat_campus++;
		}
		printf("%d\n",cat_campus);
	}
	return 0;

}