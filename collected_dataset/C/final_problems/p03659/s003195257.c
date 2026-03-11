#include <stdio.h>

int v[212345];

int main(){
    int n;
    long long resposta, total = 0;
    
    scanf("%d", &n);
    int v[n];
    
    for (int i = 0; i < n; i++){
        scanf("%d", &v[i]);
        total += v[i];
    }
    
    long long rogerio = total, mathias = 0, diferenca;
    for (int i = 0; i <= n-2; ++i){
        mathias += v[i];
        rogerio -= v[i];
        diferenca = mathias - rogerio;
        
        if (diferenca < 0)
            diferenca *= -1;
        if (i == 0 || (i != 0 && resposta > diferenca))
            resposta = diferenca;
    }
    printf("%lld\n", resposta);
    return 0;
}
