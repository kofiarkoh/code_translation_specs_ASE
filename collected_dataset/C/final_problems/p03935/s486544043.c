#include <unistd.h>

char ibuf[600030];
char *ibufe = ibuf-1;

void readall(){
  int k, t = 0;
  while((k=read(STDIN_FILENO, ibuf+t, sizeof(ibuf)-t))>0) t += k;
}

int read_uint(){
  int x=0;
  while(*(++ibufe) <'0');
  do {
    x *= 10;
    x += *ibufe-'0';
  } while(*(++ibufe) >='0');
  return x;
}

long long int read_uintll(){
  long long int x=0;
  while(*(++ibufe) <'0');
  do {
    x *= 10;
    x += *ibufe-'0';
  } while(*(++ibufe) >='0');
  return x;
}


char buf[1000];
char *bufe = buf;

void write_uintln(int x){
  int i;
  static char tmp[13];
  if(x==0){
    *bufe++ = '0';
    *bufe++ = '\n';
    return;
  }

  for(i=0; x; i++){
    tmp[i] = '0' + x % 10;
    x /= 10;
  }
  for(i--; i >= 0; i--){
    *bufe++ = tmp[i];
  }
  *bufe++ = '\n';
}

void writeall(){
  int k, t = 0;
  while((k=write(STDOUT_FILENO, buf+t, bufe-buf-t))>0) t += k;
}

#define K 19
#define M (1<<K)

typedef unsigned Zp;

const int mod = 998244353;
const Zp root = 363395222;



Zp A[M];
Zp C[M];

Zp w[M>>1];
Zp y[M>>1];
Zp v[M>>1];


Zp add(Zp lhs, Zp rhs){
  Zp r = lhs + rhs;
  if(r >= mod) return r - mod;
  else return r;
}

Zp sub(Zp lhs, Zp rhs){
  if(lhs >= rhs) return lhs - rhs;
  else return mod + lhs - rhs;
}

Zp mul(Zp lhs, Zp rhs){
  return ((long long) lhs * rhs) % mod;
}

Zp powr(Zp x, Zp n){
  Zp r = 1;
  for(; n; n>>=1){
    if(n&0x1) r = mul(r, x);
    x = mul(x, x);
  }
  return r;
}

Zp inv(Zp x){
  return powr(x, mod-2);
}

Zp tmp[M];

void fft(Zp *A, int k){
  int i, j, jh, je;
  const int m = 1 << k;
  int u = 1;
  int v = m/2;
  for(i=k;i>0;i--){
    for(jh=0;jh<u;jh++){
      Zp wj = w[jh];
      for(j = jh << i, je = j+v;j<je; j++){
        Zp Ajv = mul(wj, A[j+v]);
        A[j+v]  = sub(A[j], Ajv);
        A[j]   = add(A[j], Ajv);
      }
    }
    u <<= 1;
    v >>= 1;
  }
}


void ifft(Zp *A, int k){
  int i, j, jh, je;
  const int m = 1 << k;
  int u = m/2;
  int v = 1;
  for(i=1;i<=k;i++){
    for(jh=0;jh<u;jh++){
      Zp wj = y[jh];
      for(j = jh << i, je = j+v;j<je; j++){
        Zp Ajv = sub(A[j], A[j+v]);
        A[j] = add(A[j], A[j+v]);
        A[j+v] = mul(wj, Ajv);
      }
    }
    u >>= 1;
    v <<= 1;
  }
}

void genwy(int i, int b, Zp z, Zp x){
  if(b == 0) {
    w[i] = z;
    y[i] = x;
  }
  else {
    genwy(i, b>>1, z, x);
    genwy(i|b, b>>1, mul(z, w[b]), mul(x, y[b]));
  }
}

int n;
long long int m;

/*
[x^m] 1/((1-x)^{n-1}(1-x-x^2))
*/

int main(){
  int i, j;
  readall();
  n = read_uint();
  m = read_uintll()-1;
  if(m == 0){
    write_uintln(1);
    writeall();
    return 0;
  }
  for(i=0;i<M;i++){
    A[i] = 1;
  }

  Zp z=root;
  Zp x=inv(z);
  for(j=M/4; j; j>>=1){
    w[j] = z;
    z = mul(z, z);
    y[j] = x;
    x = mul(x, x);
  }
  genwy(0, M/4, 1, 1);

  for(i=0;i<M/2;i++){
    Zp t = w[i];
    Zp s = sub(0, w[i]);
    C[i<<1] = mul(powr(sub(1, t), n-1), sub(sub(1, t), mul(t, t)));
    C[(i<<1)|1] = mul(powr(sub(1, s), n-1), sub(sub(1, s), mul(s, s)));
  }

  const Zp im = inv(M);
  const Zp im2 = add(im, im);
  Zp ima = 1;

  Zp wi = im2;
  for(i=0;i<M/2;i++){
    v[i] = wi;
    wi = mul(wi, root);
  }

  for(; ; m >>= 1){
    for(i=0;i<M;i++){
      A[i] = mul(A[i], C[i^1]);
    }
    if(m&1){
      for(i=0;i<M/2;i++){
        A[i] = sub(A[2*i], A[2*i+1]);
        A[i] = mul(y[i], A[i]);
      }
    }
    else {
      for(i=0;i<M/2;i++){
        A[i] = add(A[2*i], A[2*i+1]);
      }
    }
    if(m==1){
      ifft(A, K-1);
      ima = mul(ima, im);
      break;
    }
    for(i=0;i<M/2;i++){
      A[M/2+i] = A[i];
    }
    ifft(A+M/2, K-1);
    for(i=0;i<M/2;i++){
      A[M/2+i] = mul(v[i], A[M/2+i]);
    }
    fft(A+M/2, K-1);
    ima = ((ima&1)? (mod+ima):ima)/2;

    for(i=0;i<M/2;i++){
      C[i] = mul(C[2*i], C[2*i+1]);
    }
    for(i=0;i<M/2;i++){
      C[M/2+i] = C[i];
    }
    ifft(C+M/2, K-1);
    for(i=0;i<M/2;i++){
      C[M/2+i] = mul(v[i], C[M/2+i]);
    }
    fft(C+M/2, K-1);
  }

  write_uintln(mul(A[0], ima));
  writeall();
  return 0;
}
