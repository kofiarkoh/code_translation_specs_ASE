#include<stdio.h>
#include<stdlib.h>
//#define MAX 51
#define MAX 2147483647

//int h[MAX];

/*
void reset(int *p){
    int i;
    for(i = 0;i < MAX;i++)p[i]=0;
}
*/

struct cell {
    int s;//start_time
    int e;//end_time
    int flag;
    int w;//weight
}p[100];

int compare(const void *a,const void *b){
    return ((struct cell *)a) -> s - ((struct cell *)b) -> s;
}

int main(){
    int sum,n,flag,m,a,b,i,j,cnt,now;

    while(scanf("%d",&n),n){
        //reset(h);
        for(i = 0;i < n;i++){
            scanf("%d%d%d",&m,&a,&b);
            p[i].w = m;
            p[i].s = a;
            p[i].e = b;
            p[i].flag = 0;
            //h[a[i]] += m[i];
            //h[b[i]] -= m[i];
        }

        qsort(p,n,sizeof(struct cell),compare);

        for(i = 0,flag = 0,now = 0;i < n;i++){

            for(j = 0;j < i;j++){
                if(p[j].flag == 1&&p[j].e<=p[i].s){
                    p[j].flag = 0;
                    now -= p[j].w;
                }
            }

            now += p[i].w;
            p[i].flag = 1;

            if(150 < now){
                flag = 1;
                break;
            }
        }

        /*
        for(i = 0,flag = 0,sum = 0,cnt = 0;i < MAX;i++){
            for(j = 0;j < n;j++){
                if(a[j] == i){
                    sum += m[j];
                }
                if(b[j] == i){
                    sum -= m[j];
                    cnt++;
                }
                //sum += h[i];
                if(sum > 150){
                    flag=1;
                    break;
                    break;
                }
            }
            if(cnt == n)break;
        }
        */
        if(flag)puts("NG");
        else puts("OK");
    }
    return 0;
}