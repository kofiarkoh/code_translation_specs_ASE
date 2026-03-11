#include <stdio.h>
#include <stdlib.h>
#define max(a,b) ((a)>(b)?(a):(b))
#define min(a,b) ((a)<(b)?(a):(b))
typedef struct Data{
    long long b,c;
}data;
int cmp(const void *a,const void *b){
  long long *A = (long long *)a;
  long long *B = (long long *)b;
  if (*A > *B) return 1;
  if (*A < *B) return -1;
  return 0;
}
int cmpb(const void *a,const void *b){
  long long A = ((data *)b)->c;
  long long B = ((data *)a)->c;
  if (A > B) return 1;
  if (A < B) return -1;
  return 0;
}
int main(){
    long long n,m,i,j;
    scanf("%lld%lld",&n,&m);
    long long a[n],sum=0;
    data num[m];
    for(i=0;i<n;i++) scanf("%lld ",&a[i]);
    qsort(a,n,sizeof(long long),cmp);
    for(i=0;i<m;i++) scanf("%lld %lld",&num[i].b,&num[i].c);
    qsort(num,m,sizeof(data),cmpb);
    i=0;
    for(j=0;j<m;j++){
            while(num[j].b>0){
                if(a[i]<num[j].c){
                    sum+=num[j].c;
                    num[j].b--;
                    i++;
                }
                else break;
            }
    }
    for(;i<n;i++)sum+=a[i];
    printf("%lld",sum);
}