# O que é isso?
É um bot para Telegram que bane ao entrar em um grupo qualquer conta com o nome "HV Cursos", que é famosa por causar spam ao entrar muitas vezes num mesmo grupo.

Eu fiz isso somente como brincadeira e para aprender Rust. Se você for da HV Cursos, não fiquem bravos comigo e por favor não façam mais spam.

# Como funciona?
Esse código escrito em Rust é compilado para Wasm e executado no Cloudflare Workers, plataforma [serverless](https://www.cloudflare.com/pt-br/learning/serverless/what-is-serverless) da Cloudflare.

Toda solicitação HTTP recebida é passada a uma função em uma nova instância ou uma existente, que irá processá-la e enviar uma resposta.

O código é executado de forma isolada pela engine V8 e não há garantia de estado compartilhado em instâncias por solicitações diferentes.

# Como usar?
## Clonar o repositório

## [Criar uma conta no Cloudflare Workers](https://dash.cloudflare.com/sign-up/workers)

## [Instalar a toolchain Rust](https://www.rust-lang.org/pt-BR/tools/install)
  
  Certifique-se de que tenha o rustc e cargo instalados.

## Instalar a ferramenta wrangler

  Recomendo instalar pelo npm, seguindo as [instruções oficiais](https://developers.cloudflare.com/workers/cli-wrangler/install-update). Pelo Cargo também é possível, mas talvez seja necessário compilar.

## Fazer login com o wrangler

  Veja as instruções na [documentação do wrangler no site oficial](https://developers.cloudflare.com/workers/cli-wrangler/commands#login).

  Uma janela será aberta em seu navegador para fazer o login.

## Editar o arquivo wrangler.toml

  Só é necessário alterar a chave **account_id**, que pode ser obtida com o comando `wrangler whoami` ou pelo [dashboard da Cloudflare](https://dash.cloudflare.com).

  As chaves seguintes são _opcionais_:

  **name**: definirá o nome e subdomínio secundário (name.seu-subdominio.workers.dev) do seu serviço.

  **compatibility_date**: talvez você queira alterar para uma data mais recente, sob o risco de causar problemas.

  O resto dos valores é recomendado _manter_, a menos que saiba o que está fazendo.

## [Criar bot no Telegram](https://t.me/botfather)

  Salve o token em algum lugar seguro, precisaremos dele para o próximo passo.

## Adicionar secrets via wrangler

  Secrets são como variáveis de ambiente, mas são valores confidenciais, por isso são tratados de forma diferente e armazenados de forma segura pela Cloudflare.
  
  Os valores que serão definidos a seguir são obrigatórios para que o bot funcione.
  
  Digite o comando e pressione Enter para inserir o valor.

  ```bash
    wrangler secret put BOT_TOKEN
  ```
  token obtido ao criar o bot no Telegram

  ```bash
    wrangler secret put WORKER_URL
  ```
  URL de acordo com chave **name** definida no arquivo wrangler.toml, ex: https://name.seu-subdominio.workers.dev

## Publicar o código

  ```bash
    wrangler publish
  ```

## Configurar webhook

  Acesse a URL https://name.seu-subdominio.workers.dev/BOT_TOKEN (substitua pelo seu token) no navegador para enviar ao Telegram as informações do webhook. Se tudo correr bem, receberá uma resposta de sucesso no navegador em formato JSON.

## Finalizar configuração

  O bot está configurado. Para testá-lo você pode criar um bot de teste com o nome "HV Cursos" e adicionar a um grupo onde o bot seja administrador.

  Ele só funciona se for administrador do grupo, isso é uma restrição do Telegram.
