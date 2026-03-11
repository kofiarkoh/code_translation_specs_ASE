#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#define ll long long
#define rep(i,l,r)for(ll i=(l);i<(r);i++)
ll po(ll a,int n){ll x=1;while(n--)x*=a;return x;}
ll pom(ll a,ll n,int m){ll x=1;for(a%=m;n;n/=2)n&1?x=x*a%m:0,a=a*a%m;return x;}
#define invp(a,p)pom(a,p-2,p)

ll exgcd(ll p,ll q,ll*x,ll*y){
	ll tx,ty,t,x2=0,y2=1;
	*x=1;*y=0;
	while(q){
		tx=*x-p/q*x2;*x=x2;x2=tx;
		ty=*y-p/q*y2;*y=y2;y2=ty;
		t=p%q;p=q;q=t;
	}
	return p;
}

//非素数での逆元
ll inv(ll a,ll p){
	ll x,y;
	ll d=exgcd(a,p,&x,&y);
	if(d!=1)return-1;
	return x>0?x:x+p;
}

ll garner(ll*r,ll*m,int n,int M){
	ll *k=(ll*)calloc(n,sizeof(ll));
	rep(i,0,n){
		ll a=0,b=1;
		rep(j,0,i){
			a=(a+b*k[j])%m[i];
			b=(b*m[j])%m[i];
		}
		//a+k*b = r[i] mod m[i]となるkを求める
		//k=(r[i]-a)/b mod m[i];
		k[i]=(r[i]-a)*inv(b,m[i])%m[i];
	}
	ll a=0,b=1;
	rep(i,0,n){
		a=(a+b*k[i])%M;
		b=(b*m[i])%M;
	}
	return (a+M)%M;
}

//きたまさ法（線形漸化式の第n項(0-based)をmodmで高速に求める）
int kitamasa(ll*coe,ll*A,int size,ll n,int m){
	/*
	例：フィボナッチ数列
	size=2;//特性方程式の次数
	coe[]={1,1};//特性方程式の最高次の係数を-1にして、消したもの
	A[]={0,1};//初期値
	*/
 
	ll*temp=(ll*)calloc(2*size,sizeof(ll));
	ll*s=(ll*)calloc(size,sizeof(ll));
	ll*ans=(ll*)calloc(size,sizeof(ll));
 
	s[1]=1;
	ans[0]=1;
 
	while(n){
		if(n%2){
			rep(i,0,2*size)temp[i]=0;
			rep(i,0,size)rep(j,0,size)temp[i+j]=(temp[i+j]+ans[i]*s[j])%m;
			for(ll i=2*size-2;i>=size;i--)rep(j,0,size)temp[i-size+j]=(temp[i-size+j]+temp[i]*coe[j])%m;
			rep(i,0,size)ans[i]=temp[i];
		}
		rep(i,0,2*size)temp[i]=0;
		rep(i,0,size)rep(j,0,size)temp[i+j]=(temp[i+j]+s[i]*s[j])%m;
		for(ll i=2*size-2;i>=size;i--)rep(j,0,size)temp[i-size+j]=(temp[i-size+j]+temp[i]*coe[j])%m;
		rep(i,0,size)s[i]=temp[i];
		n/=2;
	}
	ll ss=0;
	rep(i,0,size)ss=(ss+ans[i]*A[i])%m;
	free(temp);
	free(s);
	free(ans);
	return ss;
}

//Berlekamp-Massey algorithm(mod p)
//n個の項を引いてn次以下の漸化式を返す(n/2次以上なら項数不足)
//fibなら{1,-1,-1}が返る
ll*BM(ll*s,int n,int p){
	ll *c=(ll*)calloc(n*2,sizeof(ll));
	ll *cl=(ll*)calloc(n*2,sizeof(ll));
	ll *t=(ll*)calloc(n*2,sizeof(ll));
	ll L=0,l=-1,d,dl=1;
	c[0]=cl[0]=1;

	rep(i,0,n){
		d=0;
		rep(j,0,L+1)d=(d+c[j]*s[i-j])%p;
		if(d!=0){
			rep(j,0,n)t[j]=c[j];
			rep(j,0,n)c[j+i-l]=((c[j+i-l]-d*invp(dl,p)%p*cl[j])%p+p)%p;
			if(2*L<=i){
				l=i;
				rep(j,0,n)cl[j]=t[j];
				dl=d;
				L=i+1-L;
			}
		}
	}
	free(cl);
	free(t);
	return c;
}

ll tempcoe[110][4];
ll modmod[3]={1e9+7,1e9+9,1e9+21};

ll coe[110],ini[110];
ll L,A,B,M;
char s[99];
int main(){
	ll ans=0;
	scanf("%lld%lld%lld%lld",&L,&A,&B,&M);
	ll l=sprintf(s,"%lld",A);
	ll r=sprintf(s,"%lld",A+(L-1)*B);
	rep(k,l,r+1){
		ll x,y,n;
		//初項xと項数nを求める
		if(k==l)x=A;
		else{
			x=po(10,k-1)/B*B+(A%B);
			if(x<po(10,k-1))x+=B;
			if(x>=po(10,k))continue;
		}
		
		if(k==r)y=A+(L-1)*B;
		else{
			y=po(10,k)/B*B+(A%B);
			if(y>=po(10,k))y-=B;
		}
		n=(y-x)/B+1;
		
		ll temp=0;
		if(n<100){
			//愚直にやる
			rep(i,0,n)temp=(temp*pom(10,k,M)+(x+B*i))%M;
		}else{
			//BM+garner+kitamasa
			
			//BM*3
			rep(i,0,3){
				ll a[110];
				a[0]=x%modmod[i];
				rep(j,1,100)a[j]=(a[j-1]*pom(10,k,modmod[i])+(x+B%modmod[i]*j))%modmod[i];
				ll*p=BM(a,100,modmod[i]);
				rep(j,0,100)tempcoe[j][i]=p[j];
				free(p);
			}
			int c=99;
			while(!tempcoe[c][0]&&!tempcoe[c][1]&&!tempcoe[c][2])c--;
			assert(c<50);
			
			rep(i,0,3){
				//monicにする
				if(tempcoe[0][i]!=1){
					rep(j,1,c+1)tempcoe[j][i]=tempcoe[j][i]*invp(tempcoe[0][i],modmod[i])%modmod[i];
					tempcoe[0][i]=1;
				}
			}

			//ずらす
			rep(i,0,c+1)rep(j,0,3)tempcoe[i][j]=(tempcoe[i][j]+pom(10,20,modmod[j]))%modmod[j];
			//garner
			rep(i,0,c+1)tempcoe[i][3]=garner(tempcoe[i],modmod,3,M);			
			//戻して
			rep(i,0,c)coe[i]=(M-(tempcoe[c-i][3]-pom(10,20,M))%M)%M;
			//kitamasa
			ini[0]=x%M;
			rep(i,1,100)ini[i]=(ini[i-1]*pom(10,k,M)+(x+B*i))%M;
			temp=kitamasa(coe,ini,3,n-1,M);
		}
		
		ans=(ans*pom(pom(10,n,M),k,M)+temp)%M;
	}

	printf("%lld",ans);
}