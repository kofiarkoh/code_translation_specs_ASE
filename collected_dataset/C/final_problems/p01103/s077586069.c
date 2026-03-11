#include <stdio.h>

int main(){
	
	int d,w,e[10][10];

	int x,y,x2,y2,v,max,dote,soko;

	int i,i2;

	for(;;){//データセット

		scanf("%d %d",&d,&w);
		if(d==0)break;

		for(i=0;i<d;i++){
			for(i2=0;i2<w;i2++)scanf("%d",&e[i][i2]);
		}

		max=0;

		for(x=0;x<=d-3;x++){//左上座標
		for(y=0;y<=w-3;y++){
		for(x2=x+2;x2<d;x2++){//右下座標
		for(y2=y+2;y2<w;y2++){

					dote=9;
					soko=0;

					for(i=x;i<=x2;i++){//高さ確認
					for(i2=y;i2<=y2;i2++){

						if(i==x || i==x2 || i2==y || i2== y2){

							if(e[i][i2]<dote)dote=e[i][i2];
							continue;
						}

						if(e[i][i2]>soko)soko=e[i][i2];
					}
					}
//printf("s %d",soko);
//printf("d %d",dote);


					if(dote>soko){//容量
						v=0;
						for(i=x+1;i<x2;i++){
						for(i2=y+1;i2<y2;i2++){
							v+=dote-e[i][i2];
						}
						}
//printf("%d %d\n",e[][],e[][]);
						if(max<v)max=v;

					}
		}
		}
		}
		}

		printf("%d\n",max);

	}




	return 0;
}
