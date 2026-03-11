/*

現状さっぱりわからない。

------------------------------------------------------------------

とりあえず、サンプルと同じ結果は出せるようになった。
しかし、AOJではコンパイルエラーと言われる。（自分の環境では問題なく動く。	Ideoneではエラーと言われた。）
どうしたものか。

------------------------------------------------------------------

ついにコンパイルエラーを脱した。
原因は、output[sp]へに代入時にdata[0]".c1"	←これ（.c1）を付け忘れていたから。	そりゃエラーも起こるか。

結果：Run time error.

今度は何がおかしいのか。（自分の環境では問題なく動く。		Ideoneでもエラーなし。）

------------------------------------------------------------------

*/

#include <stdio.h>
#include <math.h>

struct TEAM
{
	int c1;
	int m1;
	int s1;
	int m2;
	int s2;
	int m3;
	int s3;
	int m4;
	int s4;
	int time;
};
typedef struct TEAM team;

void sort00(team data[], int n);
int minindex00(team data[], int from, int to);

int main()
{
	int n[100];
	team data[100];
	int output[300];
	int i=0;
	int k=0;
	int sp=0;
	while(1){
		scanf("%d",&n[i]);
		if(n[i]==0){
			break;
		}
		for(k=0; k<n[i]; k++){
			scanf("%d %d %d %d %d %d %d %d %d",&data[k].c1, &data[k].m1, &data[k].s1, &data[k].m2, &data[k].s2, &data[k].m3, &data[k].s3, &data[k].m4, &data[k].s4);
			data[k].time=((data[k].m1+data[k].m2+data[k].m3+data[k].m4)*60)+(data[k].s1+data[k].s2+data[k].s3+data[k].s4);
		}
		sort00(data, n[i]);
		output[sp]=data[0].c1;
		sp++;
		output[sp]=data[1].c1;
		sp++;
		output[sp]=data[n[i]-2].c1;
		sp++;
		i++;
	}
	for(i=0; i<sp; i++){
		printf("%d\n", output[i]);
	}
	return 0;
}

void sort00(team data[],int n)
{
	int i,minindex;
	team work;
	for(i=0;i<n;i++){
		minindex=minindex00(data,i,n);
		work=data[i];
		data[i]=data[minindex];
		data[minindex]=work;
	}
	return;
}

int minindex00(team data[], int from, int to)
{
	int i;
	int result=from;
	for(i=from;i<to;i++){
		if(data[result].time>data[i].time){
			result=i;
		}
	}
	return result;
}