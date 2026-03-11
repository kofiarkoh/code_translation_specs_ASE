
#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>



int main(void)
{
	//変数の宣言
	int n;
	int sosu[25];
	int sosucount[25];
	int over74=0;
	int over24=0;
	int over14=0;
	int over4=0;
	int over2=0;
	
	//よく使う変数
	int i,j,k,l;
	int flag=0;
	int ans=0;
	int count=0;
	long int temp,temp1,temp2;
	int max,min;
	int len;
	//データの読み込み
	
	scanf("%d",&n);
	
	sosu[0]=2;
	sosu[1]=3;
	sosu[2]=5;
	sosu[3]=7;
	sosu[4]=11;
	sosu[5]=13;
	sosu[6]=17;
	sosu[7]=19;
	sosu[8]=23;
	sosu[9]=29;
	sosu[10]=31;
	sosu[11]=37;
	sosu[12]=41;
	sosu[13]=43;
	sosu[14]=47;
	sosu[15]=53;
	sosu[16]=59;
	sosu[17]=61;
	sosu[18]=67;
	sosu[19]=71;
	sosu[20]=73;
	sosu[21]=79;
	sosu[22]=83;
	sosu[23]=89;
	sosu[24]=97;
	
	for(i=0;i<25;i++){
		sosucount[i]=0;
	}
	
//	printf("nは%dです\n", n);
//	printf("データの読み込み終了\n");
	//実際の処理
	
	for(i=1;i<=n;i++){
		temp=i;
		for(j=0;j<25;j++){
			while(temp%sosu[j]==0){
//				printf("i=%d,j=%d,temp=%d\n",i,j,temp);
				sosucount[j]++;
				temp=temp/sosu[j];
			}
		}
	}
	
//	printf("tag1\n");
	
	for(i=0;i<25;i++){
		if(sosucount[i]>=74) over74++;
		if(sosucount[i]>=24) over24++;
		if(sosucount[i]>=14) over14++;
		if(sosucount[i]>=4) over4++;
		if(sosucount[i]>=2) over2++;
	}
	
//	for(i=0;i<25;i++){
//		printf("%dは%d個\n",sosu[i],sosucount[i]);
//	}
	
//	printf("over74=%d\n",over74);
//	printf("over24=%d\n",over24);
//	printf("over14=%d\n",over14);
//	printf("over4=%d\n",over4);
//	printf("over2=%d\n",over2);
	
	ans=over74;
//	printf("tag1 ans=%d\n",ans);
	ans=ans+over24*(over2-1);
//	printf("tag2 ans=%d\n",ans);
	ans=ans+over14*(over4-1);
//	printf("tag3 ans=%d\n",ans);
	ans=ans+over4*(over4-1)*(over2-2)/2;
//	printf("tag4 ans=%d\n",ans);
	
//	printf("計算部分終了\n");
	//出力
	
	printf("%d",ans);
	
//	printf("結果の出力終了\n");
	
	return 0;
	
}

