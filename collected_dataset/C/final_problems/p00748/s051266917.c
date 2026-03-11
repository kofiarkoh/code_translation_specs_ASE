#include<stdio.h>
#define N 1000000

//"all" is the table of the answers for all pollock numbers
//"odd" is the table of the answers for odd pollock numbers
int all[N+1],odd[N+1];
int pollock[1000],num;

int BackTrack(int n,int type){
  int i,tmp,ans = N+1;
  //if n is already searched, return answer on table
  if(type && odd[n]<=N)return odd[n];
  if(!type && all[n]<=N)return all[n];

  for(i=num-1;i>0;i--){
    if(pollock[i]>n)continue;
    if(type && !(pollock[i]&1) )continue;
    tmp = BackTrack(n-pollock[i],type);
    if(ans > tmp+1)ans = tmp+1;
  }

  //memorize n-th answer
  if(type)odd[n] = ans;
  else all[n] = ans;
  return ans;
}

int main(void){
  int i,n,tmp;

  //initialize table (filling SUP)
  for(i=0;i<=N;i++)all[i] = odd[i] = N+1;
  all[0] = odd[0] = 0;

  for(num=1;;num++){
    tmp = num*(num+1)*(num+2)/6;
    if(tmp>N)break;
    pollock[num] = tmp;
  }

  while(scanf("%d",&n) && n){
    printf("%d %d\n",BackTrack(n,0),BackTrack(n,1));
  }
  return 0;
}