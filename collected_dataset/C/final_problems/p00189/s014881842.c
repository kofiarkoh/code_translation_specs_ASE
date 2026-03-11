#include<stdio.h>
int min(int a,int b){
    if(a>b)return b;
    else return a;
}
int main(){
    int d[10][10];
    int i,j,k,n,s,ans1,ans2;
    int v;
    v=0;
    s=0;ans1=0;ans2=-1;
    int t1,t2,t3;
    while(1){
        scanf("%d",&n);
        if(n==0)break;
        for(i=0;i<=9;i++){
            for(j=0;j<=9;j++)if(i!=j)d[i][j]=100000;else d[i][j]=0;
	    }
        for(i=1;i<=n;i++){
            scanf("%d%d%d",&t1,&t2,&t3);
            d[t1][t2]=t3;d[t2][t1]=t3;
            if(t1>v)v=t1;if(t2>v)v=t2;
        }
        for(k=0;k<=v;k++)
            for(i=0;i<=v;i++)
                for(j=0;j<=v;j++)d[i][j]=min(d[i][j],d[i][k]+d[k][j]);
        for(i=0;i<=v;i++){
            for(j=0;j<=v;j++)s=s+d[i][j];
            if(ans2<0||s<ans2){ans1=i;ans2=s;}
            s=0;
        }
        printf("%d %d\n",ans1,ans2);
        v=0;ans1=0;ans2=-1;s=0;
    }
    return 0;
}

