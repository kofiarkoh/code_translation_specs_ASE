/*2019/08/25 */
#include<stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h>
#define MOD 1000000007
#define MIN 999999999
#define MAX -999999999
typedef long long int ll;
typedef unsigned long long int ull;
void llswap(ll *x,ll *y){ll temp;temp=*x;*x=*y;*y=temp;}
void swap(int *x,int *y){int tmp;tmp=*x;*x=*y;*y=tmp;}
int rmax(int x,int y){return x>y?x:y;}
int rmin(int x,int y){return x>y?y:x;}
ll llrmax(ll x,ll y){return x>y?x:y;}
ll llrmin(ll x,ll y){return x>y?y:x;}
int asc(const void *a,const void *b){return *(int*)a-*(int*)b;}
int desc(const void *a,const void *b){return *(int*)b-*(int*)a;}
int llasc(const void *a,const void *b){return *(ll*)a-*(ll*)b;}
int lldesc(const void *a,const void *b){return *(ll*)b-*(ll*)a;}

typedef struct{
    int h;
    int num;
}group_t;

int asc_t(const void *a,const void *b){return ((group_t*)a)->h - ((group_t*)b)->h;}
//int desc_t(const void *a,const void *b){return ((struct_t*)b)->member - ((struct_t*)a)->member;}


int main(){
    int h[200000],n;
    scanf("%d",&n);
    
    for(int i=0;i<n;i++){
        scanf("%d",&h[i]);
    }

    int cnt=0;
    int max=MAX;
    for(int i=n-1;i>0;i--){
        if(h[i]<=h[i-1]){
            cnt++;
        }else{
            if(max<cnt){
                max=cnt;
            }
            cnt=0;
        }
    }
    if(max<cnt){
        max=cnt;
    }

    printf("%d\n",max);

    return 0;


}