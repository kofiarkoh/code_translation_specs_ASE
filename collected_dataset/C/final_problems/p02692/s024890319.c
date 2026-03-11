#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#define MOD1 1000000007
#define MOD2 998244353
#define LIMIT1 200002
#define LIMIT2 500002
typedef long long ll;
typedef long double ld;
typedef const void cv;
#define rep(i,n) for(i=0;i<n;i++)
#define max(a,b) ((a)>(b) ? (a) : (b))
#define min(a,b) ((a)<(b) ? (a) : (b))
#define zt(a,b) (max((a),(b))-min((a),(b)))
ll gcd(ll a,ll b){return b?gcd(b,a%b):a;}
ll nPr(int n,int r){ll i,result=1;rep(i,r){result*=(ll)(n-i);}return result;}
ll nCr(int n,int r){ll i,result=1;if(n<r) return 0;rep(i,min(r,n-r)){result*=(ll)(n-i);result/=(ll)(i+1);}return result;}
#define fact(n) nPr((int)(n),(int)(n))
#define nHr(n,r) nCr((int)((n)+(r)+1),(int)(r))
#define sankaku(x) (((x)*((x)+1))/2)
int dx[8]={1,0,-1,0,1,-1,-1,1};
int dy[8]={0,1,0,-1,1,1,-1,-1};
int upint(const void *a, const void *b) { return *(int *)a < *(int *)b ? -1 : *(int *)a > *(int *)b ? 1 : 0; }
int downint(const void *a, const void *b) { return *(int *)a < *(int *)b ? 1 : *(int *)a > *(int *)b ? -1 : 0; }
int upchar(const void* left, const void* right) {return strcmp((char *)left,(char *)right);}
int downchar(const void* left, const void* right) {return strcmp((char *)right,(char *)left);}
ll modpow(ll a, ll n, ll mod) {
  ll res = 1;
  while (n > 0) {
    if (n & 1) res = res * a % mod;
    a = a * a % mod;
    n >>= 1;
  }
  return res;
}
void initialize(){
    
}
int a[200000]={0};
int main(void){
  initialize();
  ll n,m,i,j,k,result=1,a,b,c;
  char s[LIMIT1][10];
  char choice[LIMIT1]="";
  ll ch1,ch2;
  int cc[3]={0};
  scanf("%lld %lld %lld %lld",&n,&a,&b,&c);
  rep(i,n){
    scanf("%s",s[i]);
  }
  
  rep(i,n){
    if(s[i][0]=='A'){
      ch1=a;
      cc[0]++;
      //printf("%lld ",a);
    }else{
      ch1=b;
      cc[1]++;
      //printf("%lld ",b);
    }
    if(s[i][1]=='B'){
      ch2=b;
      cc[1]++;
      //printf("%lld\n",b);
    }else{
      ch2=c;
      cc[2]++;
      //printf("%lld\n",c);
    }
    if(ch1==0&&ch2==0){
      result=0;
      break;
    }
    if(ch1==0){
      choice[i]=s[i][0];
      switch(choice[i]){
        case 'A':
          a++;
          break;
        case 'B':
          b++;
          break;
        default:
          c++;
          break;
      }
    }else if(ch2==0){
      choice[i]=s[i][1];
      switch(choice[i]){
        case 'A':
          a++;
          break;
        case 'B':
          b++;
          break;
        default:
          c++;
          break;
      }
    }else{
      if(i<n-1){
        if(s[i+1][0]=='A'){
          cc[0]+=2;
        }else{
          cc[1]+=2;
        }
        if(s[i+1][1]=='B'){
          cc[1]+=2;
        }else{
          cc[2]+=2;
        }
      }else{
        cc[0]+=2;
        cc[1]+=2;
      }
      if(cc[0]>=3){
        choice[i]='A';
        a++;
      }else if(cc[1]>=3){
        choice[i]='B';
        b++;
      }else{
        choice[i]='C';
        c++;
      }
    }
    rep(j,3){
      cc[j]%=2;
      if(cc[j]==1 && choice[i]!=(char)('A'+j)){
        if(j==0) a--;
        else if(j==1) b--;
        else c--;
      }
      cc[j]=0;
    }
    //printf("%c\n",choice[i]);
  }
  
  if(result==0){
    printf("No");
  }else{
    printf("Yes\n");
    rep(i,n){
      printf("%c\n",choice[i]);
    }
  }
  return 0;
}
