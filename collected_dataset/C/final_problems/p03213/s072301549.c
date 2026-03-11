#include <stdio.h>
#include<stdlib.h>
#include<math.h>
#include<string.h> 
#define ll unsigned long long
#define length 100000   //10万
#define kw_len 10
ll factor[1000]; 
ll memory[1000];
int primefac(ll num){
    int i = 0,kosuu=0,flag=0,sum,n=0;
    while(*(factor+i) != 0){
        sum =1;
        flag=0;
        while(num % *(factor+i)==0){
            num /= *(factor+i);
            memory[*(factor+i)] +=1;
        }
        i++;
    }
    return sum;
}

/*グローバルにあるfactorに素数書き込む.個数を返すので配列は-1しろ*/
int elast(ll number){    
    ll tmp=0;
    int flag=0;
    *factor = 2;
    for(int i=3;i <= number;i++){
        flag=0;
        for(int j=0;j <= tmp ;j++){
            if(i % (*(factor+j)) == 0){
                flag=1;
                break;
            }
        }
        if(flag)continue;
        tmp += 1;
        *(factor+tmp) = i;
    }
        return tmp+2;
}



ll nCr(int n,int r){
  ll val=1;
  int i;
  if(r==0)return 1;
  for(i=0;i<n-r;i++){
		val *= (n-i);
		val /= (i+1);
	}
  return val;
}

ll nPr(int n,int r){
  ll val=1;
  int i;
  for(i=0;i<r;i++){
    val *= (n-i);
  }
  return val;
}

int main(){
    ll three=0,nago=0,nigo=0,igo=0;
    ll five=0;
    ll N,i,j;
    ll sum=0;
    memset(factor ,0, sizeof(factor));
    memset(memory,0,sizeof(memory));
    scanf("%llu",&N);
    elast(N);
    for(i=1;i<N;i++){
      primefac(i+1);
    }
    for(i=0;i<N;i++){
      if(memory[i]+1>=75)nago++;
      if(memory[i]+1>=25)nigo++;
      if(memory[i]+1>=15)igo++;
      if(memory[i]+1>=5){
        five++;
        three++;
      }
      else if(memory[i]+1>=3){
        three++;
      }
    }
    if(three>=2){
     sum += nCr(five,2)*(three-2);
     sum +=nago;
     sum += nigo*(three-1);
     sum += igo*(five-1);
    }
    printf("%llu",sum);

    return 0;
}
