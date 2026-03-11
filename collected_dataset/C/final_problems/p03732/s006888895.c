#include<stdio.h>
#include<string.h>
#include<math.h>
#define FOR(i,a,b) for(int i=a;i<b;i++)
#define ll long long
#define swap(a,b) {a+=b;b=a-b;a-=b;}
ll N,W, t1[102][2], t2[102][2], t3[102][2], t4[102][2];
ll maxv = 0;

int main(){
	ll wi,vi,w1 = -1;
	int t1n = 0,t2n = 0,t3n = 0,t4n = 0;

	//input
	scanf("%lld %lld",&N,&W);

	FOR(i,0,N){
		scanf("%lld %lld",&wi,&vi);
		if(w1 == -1) w1 = wi;

		if(wi == w1)	{ t1[t1n][0] = wi; t1[t1n][1] = vi; t1n++; }
		if(wi == w1 + 1){ t2[t2n][0] = wi; t2[t2n][1] = vi; t2n++; }
		if(wi == w1 + 2){ t3[t3n][0] = wi; t3[t3n][1] = vi; t3n++; }
		if(wi == w1 + 3){ t4[t4n][0] = wi; t4[t4n][1] = vi; t4n++; }
	}

	//価値0も加える
	t1[t1n][0] = w1; 		t1[t1n][1] = 0; t1n++;
	t2[t1n][0] = w1 + 1; 	t2[t1n][1] = 0; t2n++;
	t3[t1n][0] = w1 + 2; 	t3[t1n][1] = 0; t3n++;
	t4[t1n][0] = w1 + 3; 	t4[t1n][1] = 0; t4n++;

	//価値の大きい順にソート
	FOR(i,0,t1n-1) if(t1[i][1] < t1[i+1][1]) swap(t1[i][1],t1[i+1][1]);
	FOR(i,0,t2n-1) if(t2[i][1] < t2[i+1][1]) swap(t2[i][1],t2[i+1][1]);
	FOR(i,0,t3n-1) if(t3[i][1] < t3[i+1][1]) swap(t3[i][1],t3[i+1][1]);
	FOR(i,0,t4n-1) if(t4[i][1] < t4[i+1][1]) swap(t4[i][1],t4[i+1][1]);

	//t1~t4それぞれから入れる数を決定
	ll tw = 0, tv= 0;

	FOR(i,0,t1n){
		FOR(j,0,t2n){
			FOR(k,0,t3n){
				FOR(l,0,t4n){

					tw = w1*i + (w1+1)*j + (w1+2)*k + (w1+3)*l;
					if(tw <= W){
						FOR(m,0,i) tv += t1[m][1];
						FOR(m,0,j) tv += t2[m][1];
						FOR(m,0,k) tv += t3[m][1];
						FOR(m,0,l) tv += t4[m][1];
						if(tv > maxv) maxv = tv;
					}
					tw = 0; tv = 0;

				}
			}
		}
	}

	printf("%lld\n",maxv);
}