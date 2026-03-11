#include<stdio.h>
#include<stdlib.h>

#define gc getchar
int scan(int*N)
{
	int c,n=0;
	for(c=gc();'0'<=c&&c<='9';c=gc())
	    n=(n*10)+(c&0xf);
	if(N!=NULL)*N=n;
	return n;
}

/*設問用*/
#define N_MAX 3000
typedef struct
{
    int x;
    int y;
}Point;

/*グローバル変数*/
int N;          //座標平面上の点の数
int K;          //同じ直線上にあってはならない点の数
Point P[N_MAX]; //点の座標データ
Point C[N_MAX]; //ある点P[i]を中心にした座標データ

/*クイックソート用比較関数*/
int comp(const void *a, const void *b)
{
    return ((Point *)b)->x * ((Point *)a)->y - ((Point *)a)->x * ((Point *)b)->y;
}

int cnt(int n)
{
    Point c;
    c = C[0];
    int L=2;    //直線上にある点の数
    int M=2;    //上記の最大数
    
    /*傾きが近い順にソート*/
    qsort(C, n, sizeof(Point), comp);
    
    /*同じ傾きの数え上げ*/
    for(int i=1; i<n; i++){
        L += (c.x * C[i].y) == (C[i].x * c.y);//等傾
        if(L > M) M=L;//最大数更新
        else{ L=2, c=C[i]; }
    }
    return M;
}

int main()
{
    int i,j;//ループ用
    int n;//i番目以降の残りの点の数
    int v;//縦に並んでいる点の数
    int ans=0;//回答
    
    /*読込*/
    scan(&N);
	scan(&K);
    for(i=0; i<N; i++){
        scan(&P[i].x);
    	scan(&P[i].y);
    }
    
    /*P[i]中心の座標にして数える*/
    for (i=0; i<=(N-K); i++){
        v=1,n=0;
        for (j=i+1; j<N; j++){
            if ( P[i].x == P[j].x ){
                v++;
            }else{
                C[n] = P[j];
                C[n].x -= P[i].x;
                C[n].y -= P[i].y;
            	n++;
            }
        }
        if ( v>=K || cnt(n)>=K ){
            ans=1;
            break;
        }
    }
    printf("%d\n", ans);
    return 0;
}
