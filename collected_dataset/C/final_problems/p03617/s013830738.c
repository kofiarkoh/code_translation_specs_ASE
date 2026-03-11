#include<stdio.h>

void boubleSort(long int x[4]) {
  for (int i = 0; i < 3; i++) {
    for ( int j = 0; j < 3-i; j++) {
      if (x[j] > x[j+1]) {
        int tmp = x[j];
        x[j] = x[j+1];
        x[j+1] = tmp;
      }
    }
  }
}

int main(void) {
  long int Q, H, S, D, N;
  scanf("%ld %ld %ld %ld", &Q, &H, &S, &D);
  scanf("%ld", &N);
  
  long int values[4] = {8*Q, 4*H, 2*S, D};
  boubleSort(values);
  long int a, b, c, d;   //a<b<c<d 2L単位の値段
  a = values[0];
  b = values[1];
  c = values[2];
  d = values[3];
  
  //8Q, 4H, 2Sが最安値なら全てそれで購入できる
  if (a != D) printf("%ld", (N*a)/2);
  else {
    //Dが最安値ならできるだけDで買い、最後の1Lのみどれかで買う
    printf("%ld", (N/2)*a+(N%2)*b/2);
  }
  
  return 0;
}
