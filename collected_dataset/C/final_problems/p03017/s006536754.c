#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>

#define MAX(p,q) ((p>q)?(p):(q))
#define MIN(p,q) ((p<q)?(p):(q))

#define INF 1000000000;
#define REP(i,n) for(i=0;i<n;i++)

int gcd(int a,int b);
/*比較函数の実装
int -32768 to 32767 (2**15-1)
long -2147483648 to 2147483648 (2**31-1)
*/
int intcmp(const void *a,const void *b){
  return *(int *)a - *(int *)b;
}
//qsort(a,sizeof(a)/sizeof(int),sizeof(int),intcmp);
int longcmp(const void *a,const void *b){
  return *(long *)a - *(long *)b;
}
//qsort(a,sizeof(a)/sizeof(long),sizeof(long),longcmp);
int doublecmp(const void *a, const void *b){
  return *(double *)a - *(double *)b;
}
//qsort(a,sizeof(a)/sizeof(double),sizeof(double),doublecmp);
int main(void)
{
  int n,a,b,c,d;
  scanf("%d%d%d%d%d",&n,&a,&b,&c,&d);
  char s[200001];
  int i,flag1=0,flag2=0,max1=0,max2=0,fc1=0,fc2=0,mc1=0,mc2=0;
  REP(i,n)
  {
    scanf(" %c",&s[i]);
    if(s[i]=='#') {
      if(i>a-1&&i<d-1) flag1++;
      if(i>a-1&i<c-1) {
        fc1++;
      //  printf("%dのfc1は%d\n",i,fc1);
      }
      if(i>b&i<d) {
        fc2++;
        //printf("%dのfc2は%d\n",i,fc2);
      }
    }
    else{
      if(max1<flag1) max1 = flag1;
      flag1 = 0;
      if(mc1<fc1) mc1 = fc1;
      fc1 = 0;
      if(mc2<fc2) mc2 = fc2;
      fc2 = 0;
    }
    if(s[i]=='.') flag2++;
    else{
      if(max2<flag2) max2 = flag2;
      flag2 = 0;
    }
    //printf("s[%d]は%c\n",i,s[i]);


  }
  //printf("%d\t%d",mc1,mc2);
  if(c<b&&mc1<2&&mc2<2) printf("Yes\n");
  else if (max1 >= 2) printf("No\n");
  else if(c < d) printf("Yes\n");
  else if(max2 >= 3) printf("Yes\n");
  else printf("No\n");
  return 0;
}

int gcd(int a,int b)
{
  int max,min,tmp;
  max = MAX(a,b);
  min = MIN(a,b);
  if(max%min==0) return min;
  else
  {
    tmp = min;
    min = max%min;
    max = tmp;
    return gcd(max,min);
  }
}
