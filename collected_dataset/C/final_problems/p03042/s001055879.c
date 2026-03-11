#include <stdio.h>

int main(void){
    char s[5];
    int i;
    
    scanf("%s", s);
    
    switch(s[0]){
        case '0':
            if(s[1] == '0'){
                if((s[2] == '0' && s[3] >= '1') || (s[2] == '1' && s[3] <= '2')){
                    puts("YYMM");
                }else{
                    puts("NA");
                }
            }else{
                if((s[2] == '0' && s[3] >= '1') || (s[2] == '1' && s[3] <= '2')){
                    puts("AMBIGUOUS");
                }else{
                    puts("MMYY");
                }
            }
            
        
        break;
        
        case '1':
            if(s[1] <= '2'){
                if((s[2] == '0' && s[3] >= '1') || (s[2] == '1' && s[3] <= '2')){
                    puts("AMBIGUOUS");
                }else{
                    puts("MMYY");
                }
            }else{
                if((s[2] == '0' && s[3] >= '1') || (s[2] == '1' && s[3] <= '2')){
                    puts("YYMM");
                }else{
                    puts("NA");
                }
            }
        
        break;
        
        default:
        //MMXX
            if((s[2] == '0' && s[3] >= '1') || (s[2] == '1' && s[3] <= '2')){
                puts("MMYY");
            }else{
                puts("NA");
            }
        
        break;
    }
    
    return 0;
}