import { zodResolver } from '@hookform/resolvers/zod';
import type { HTMLAttributes } from 'react';
import { forwardRef, useImperativeHandle } from 'react';
import type { Control, FieldPath, UseFormReturn } from 'react-hook-form';
import { useForm, useWatch } from 'react-hook-form';
import { useTranslation } from 'react-i18next';

import {
  PROVIDER_AZURE,
  PROVIDER_CLAUDE,
  PROVIDER_CUSTOM,
  PROVIDER_DEEPSEEK,
  PROVIDER_GOOGLE,
  PROVIDER_OLLAMA,
  PROVIDER_OPENAI,
  PROVIDER_OPENROUTER,
  PROVIDER_XAI,
} from '@/lib/constants';
import {
  editAzureModelFormSchema,
  editClaudeModelFormSchema,
  editGoogleModelFormSchema,
  editOllamaModelFormSchema,
  editOpenAIModelFormSchema,
  newAzureModelFormSchema,
  newClaudeModelFormSchema,
  newGoogleModelFormSchema,
  newOllamaModelFormSchema,
  newOpenAIModelFormSchema,
} from '@/lib/schemas';
import type {
  AzureModel,
  ClaudeModel,
  GoogleModel,
  Model,
  ModelFormHandler,
  NewAzureModel,
  NewClaudeModel,
  NewGoogleModel,
  NewModel,
  NewOllamaModel,
  NewOpenAIModel,
  OllamaModel,
  OpenAIModel,
  RawClaudeConfig,
  RawConfig,
  RawGoogleConfig,
  RawOllamaConfig,
  RawOpenAIConfig,
} from '@/lib/types';

import { InputWithMenu } from '../InputWithMenu';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '../ui/form';
import { Input } from '../ui/input';
import { RemoteModelsSelector } from './RemoteModelsSelector';

type NewFormProps = Omit<HTMLAttributes<HTMLFormElement>, 'onSubmit'> & {
  onSubmit: (model: NewModel) => void;
};

type EditFormProps = Omit<HTMLAttributes<HTMLFormElement>, 'onSubmit'> & {
  model: Model;
  onSubmit: (model: Model) => void;
};

type GenericFormProps<T extends NewModel | Model> = Omit<
  HTMLAttributes<HTMLFormElement>,
  'onSubmit'
> & {
  form: UseFormReturn<T, any, undefined>;
  onSubmit: (model: T) => void;
  loadModelsOnInit?: boolean;
};

type FormFieldProps<T extends NewModel | Model> = {
  control: Control<T>;
  name: FieldPath<T>;
  label: string;
  placeholder?: string;
  tips?: string;
};

// ModelForm's input component
const InputField = <T extends NewModel | Model>({
  control,
  name,
  label,
  placeholder,
  tips,
}: FormFieldProps<T>) => {
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className="grid grid-cols-4 items-center gap-x-4 gap-y-1 space-y-0">
          <FormLabel className="text-right">{label}</FormLabel>
          <FormControl>
            <InputWithMenu
              className="col-span-3"
              {...field}
              value={(field.value ?? '') as string}
              placeholder={placeholder}
            />
          </FormControl>
          <div className="col-start-2 col-end-4">
            <FormMessage />
            {tips ? <FormDescription>{tips}</FormDescription> : null}
          </div>
        </FormItem>
      )}
    />
  );
};

// ModelForm's hidden input component
const HiddenInputField = <T extends NewModel | Model>({
  control,
  name,
}: Omit<FormFieldProps<T>, 'label' | 'placeholder'>) => {
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <InputWithMenu
              type="hidden"
              {...field}
              value={(field.value ?? '') as string}
            />
          </FormControl>
          <div className="col-span-4">
            <FormMessage />
          </div>
        </FormItem>
      )}
    />
  );
};

// ModelForm's input for model
const ModelField = <T extends NewModel | Model>({
  control,
  name,
  label,
  tips,
  config,
  loadOnInit = false,
}: Omit<FormFieldProps<T>, 'placeholder'> & {
  config: RawConfig;
  loadOnInit: boolean;
}) => {
  return (
    <div className="grid grid-cols-4 items-center gap-x-4 gap-y-1 space-y-0">
      <FormField
        control={control}
        name={name}
        render={() => <FormLabel className="text-right">{label}</FormLabel>}
      />
      <div className="col-span-3 col-start-2 flex justify-between gap-2">
        <RemoteModelsSelector config={config} enabledByDefault={!!loadOnInit} />
      </div>
      <div className="col-span-3 col-start-2">
        <FormField
          control={control}
          name={name}
          render={() => <FormMessage />}
        />
        <FormDescription>{tips}</FormDescription>
      </div>
    </div>
  );
};

const GenericAzureModelForm = ({
  form,
  onSubmit,
  ...props
}: GenericFormProps<NewModel | Model>) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="apiKey"
            label={t('page-models:label:api-key')}
            tips={t('page-models:message:api-key-tips')}
          />
          <InputField
            control={form.control}
            name="endpoint"
            label={t('page-models:label:endpoint')}
            tips={t('page-models:message:endpoint-tips-azure')}
          />
          <InputField
            control={form.control}
            name="apiVersion"
            label={t('page-models:label:api-version')}
            tips={t('page-models:message:api-version-tips')}
          />
          <InputField
            control={form.control}
            name="deploymentId"
            label={t('page-models:label:deployment-id')}
            tips={t('page-models:message:deployment-id-tips')}
          />
          <HiddenInputField control={form.control} name="provider" />
          {isEdit ? (
            <HiddenInputField control={form.control} name="id" />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const GenericOpenAIModelForm = ({
  form,
  onSubmit,
  loadModelsOnInit,
  allowEndpointEdit = false,
  allowModelSelection = false,
  ...props
}: GenericFormProps<NewOpenAIModel | OpenAIModel> & {
  allowEndpointEdit?: boolean;
  allowModelSelection?: boolean;
}) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  const provider = form.getValues('provider');
  const apiKey = useWatch({ name: 'apiKey', control: form.control });
  const config: RawOpenAIConfig = {
    provider,
    apiKey,
  };
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="apiKey"
            label={t('page-models:label:api-key')}
            tips={t('page-models:message:api-key-tips')}
          />
          {allowEndpointEdit ? (
            <InputField
              control={form.control}
              name="endpoint"
              label={t('page-models:label:endpoint')}
              tips={t('page-models:message:endpoint-tips')}
            />
          ) : null}
          {allowModelSelection ? (
            <ModelField
              control={form.control}
              name="model"
              label={t('page-models:label:model')}
              tips={t('page-models:message:model-tips')}
              config={config}
              loadOnInit={!!loadModelsOnInit}
            />
          ) : (
            <InputField
              control={form.control}
              name="model"
              label={t('page-models:label:model')}
              tips={t('page-models:message:model-tips')}
            />
          )}
          <FormField
            control={form.control}
            name="provider"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input type="hidden" {...field} />
                </FormControl>
                <div className="col-span-3 col-start-2">
                  <FormMessage />
                </div>
              </FormItem>
            )}
          />
          {isEdit ? (
            <FormField
              control={form.control}
              name="id"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input type="hidden" {...field} />
                  </FormControl>
                  <div className="col-span-3 col-start-2">
                    <FormMessage />
                  </div>
                </FormItem>
              )}
            />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const GenericClaudeModelForm = ({
  form,
  onSubmit,
  loadModelsOnInit,
  ...props
}: GenericFormProps<NewModel | Model>) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  const apiKey = useWatch({ name: 'apiKey', control: form.control });
  const apiVersion = useWatch({ name: 'apiVersion', control: form.control });
  const config: RawClaudeConfig = {
    provider: PROVIDER_CLAUDE,
    apiVersion: apiVersion ?? '',
    apiKey: apiKey ?? '',
  };
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="apiKey"
            label={t('page-models:label:api-key')}
            tips={t('page-models:message:api-key-tips')}
          />
          <ModelField
            control={form.control}
            name="model"
            label={t('page-models:label:model')}
            tips={t('page-models:message:model-tips')}
            config={config}
            loadOnInit={!!loadModelsOnInit}
          />
          <InputField
            control={form.control}
            name="apiVersion"
            label={t('page-models:label:api-version')}
            tips={t('page-models:message:claude-api-version-tips')}
          />
          <FormField
            control={form.control}
            name="provider"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input type="hidden" {...field} />
                </FormControl>
                <div className="col-span-3 col-start-2">
                  <FormMessage />
                </div>
              </FormItem>
            )}
          />
          {isEdit ? (
            <FormField
              control={form.control}
              name="id"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input type="hidden" {...field} />
                  </FormControl>
                  <div className="col-span-3 col-start-2">
                    <FormMessage />
                  </div>
                </FormItem>
              )}
            />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const GenericOllamaModelForm = ({
  form,
  onSubmit,
  loadModelsOnInit,
  ...props
}: GenericFormProps<NewModel | Model>) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  const endpoint = useWatch({ name: 'endpoint', control: form.control });
  const config: RawOllamaConfig = {
    provider: PROVIDER_OLLAMA,
    endpoint: endpoint ?? '',
  };
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="endpoint"
            label={t('page-models:label:endpoint')}
            tips={t('page-models:message:endpoint-tips')}
          />
          <ModelField
            control={form.control}
            name="model"
            label={t('page-models:label:model')}
            tips={t('page-models:message:model-tips')}
            config={config}
            loadOnInit={!!loadModelsOnInit}
          />
          <HiddenInputField control={form.control} name="provider" />
          {isEdit ? (
            <HiddenInputField control={form.control} name="id" />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const GenericGoogleModelForm = ({
  form,
  onSubmit,
  loadModelsOnInit,
  ...props
}: GenericFormProps<NewModel | Model>) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  const apiKey = useWatch({ name: 'apiKey', control: form.control });
  const config: RawGoogleConfig = {
    provider: PROVIDER_GOOGLE,
    apiKey: apiKey ?? '',
  };
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="apiKey"
            label={t('page-models:label:api-key')}
            tips={t('page-models:message:api-key-tips')}
          />
          <ModelField
            control={form.control}
            name="model"
            label={t('page-models:label:model')}
            tips={t('page-models:message:model-tips')}
            config={config}
            loadOnInit={!!loadModelsOnInit}
          />
          <FormField
            control={form.control}
            name="provider"
            render={({ field }) => (
              <FormItem>
                <FormControl>
                  <Input type="hidden" {...field} />
                </FormControl>
                <div className="col-span-3 col-start-2">
                  <FormMessage />
                </div>
              </FormItem>
            )}
          />
          {isEdit ? (
            <FormField
              control={form.control}
              name="id"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input type="hidden" {...field} />
                  </FormControl>
                  <div className="col-span-3 col-start-2">
                    <FormMessage />
                  </div>
                </FormItem>
              )}
            />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const NewAzureModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form: UseFormReturn<NewAzureModel, unknown, undefined> =
      useForm<NewAzureModel>({
        resolver: zodResolver(newAzureModelFormSchema),
        defaultValues: {
          provider: PROVIDER_AZURE,
          alias: '',
          apiKey: '',
          endpoint: '',
          apiVersion: '',
          deploymentId: '',
        },
      });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericAzureModelForm
        form={form as UseFormReturn<NewModel, unknown, undefined>}
        onSubmit={onSubmit}
        {...props}
      />
    );
  }
);

const EditAzureModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<AzureModel>({
      resolver: zodResolver(editAzureModelFormSchema),
      defaultValues: model as AzureModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericAzureModelForm
        form={form as UseFormReturn<NewModel | Model, unknown, undefined>}
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        {...props}
      />
    );
  }
);

const NewOpenAIModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOpenAIModel>({
      resolver: zodResolver(newOpenAIModelFormSchema),
      defaultValues: {
        provider: PROVIDER_OPENAI,
        alias: '',
        apiKey: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={form as UseFormReturn<NewOpenAIModel, any, undefined>}
        onSubmit={onSubmit}
        allowModelSelection
        {...props}
      />
    );
  }
);

const EditOpenAIModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<OpenAIModel>({
      resolver: zodResolver(editOpenAIModelFormSchema),
      defaultValues: model as OpenAIModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={
          form as UseFormReturn<NewOpenAIModel | OpenAIModel, any, undefined>
        }
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        allowModelSelection
        loadModelsOnInit
        {...props}
      />
    );
  }
);

const NewClaudeModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewClaudeModel>({
      resolver: zodResolver(newClaudeModelFormSchema),
      defaultValues: {
        provider: PROVIDER_CLAUDE,
        endpoint: undefined,
        alias: '',
        apiKey: '',
        model: '',
        apiVersion: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericClaudeModelForm
        form={form as UseFormReturn<NewModel, any, undefined>}
        onSubmit={onSubmit}
        {...props}
      />
    );
  }
);

const EditClaudeModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<ClaudeModel>({
      resolver: zodResolver(editClaudeModelFormSchema),
      defaultValues: model as ClaudeModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericClaudeModelForm
        form={form as UseFormReturn<NewModel | Model, any, undefined>}
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        {...props}
      />
    );
  }
);

const NewOllamaModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOllamaModel>({
      resolver: zodResolver(newOllamaModelFormSchema),
      defaultValues: {
        provider: PROVIDER_OLLAMA,
        alias: '',
        endpoint: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOllamaModelForm
        form={form as UseFormReturn<NewModel, any, undefined>}
        onSubmit={onSubmit}
        {...props}
      />
    );
  }
);

const EditOllamaModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<OllamaModel>({
      resolver: zodResolver(editOllamaModelFormSchema),
      defaultValues: model as OllamaModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOllamaModelForm
        form={form as UseFormReturn<NewModel | Model, any, undefined>}
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        loadModelsOnInit
        {...props}
      />
    );
  }
);

const NewOpenrouterModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOpenAIModel>({
      resolver: zodResolver(newOpenAIModelFormSchema),
      defaultValues: {
        provider: PROVIDER_OPENROUTER,
        alias: '',
        apiKey: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={form as UseFormReturn<NewOpenAIModel, any, undefined>}
        onSubmit={onSubmit}
        allowModelSelection
        {...props}
      />
    );
  }
);

const NewDeepseekModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOpenAIModel>({
      resolver: zodResolver(newOpenAIModelFormSchema),
      defaultValues: {
        provider: PROVIDER_DEEPSEEK,
        alias: '',
        apiKey: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={form as UseFormReturn<NewOpenAIModel, any, undefined>}
        onSubmit={onSubmit}
        allowModelSelection
        {...props}
      />
    );
  }
);

const NewXaiModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOpenAIModel>({
      resolver: zodResolver(newOpenAIModelFormSchema),
      defaultValues: {
        provider: PROVIDER_XAI,
        alias: '',
        apiKey: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={form as UseFormReturn<NewOpenAIModel, any, undefined>}
        onSubmit={onSubmit}
        allowModelSelection
        {...props}
      />
    );
  }
);

const NewGoogleModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewGoogleModel>({
      resolver: zodResolver(newGoogleModelFormSchema),
      defaultValues: {
        provider: PROVIDER_GOOGLE,
        alias: '',
        apiKey: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericGoogleModelForm
        form={form as UseFormReturn<NewModel, any, undefined>}
        onSubmit={onSubmit}
        {...props}
      />
    );
  }
);

const EditGoogleModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<GoogleModel>({
      resolver: zodResolver(editGoogleModelFormSchema),
      defaultValues: model as GoogleModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericGoogleModelForm
        form={form as UseFormReturn<NewModel | Model, any, undefined>}
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        {...props}
      />
    );
  }
);

const NewCustomModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOpenAIModel>({
      resolver: zodResolver(newOpenAIModelFormSchema),
      defaultValues: {
        provider: PROVIDER_CUSTOM,
        alias: '',
        apiKey: '',
        model: '',
        endpoint: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={form as UseFormReturn<NewOpenAIModel, any, undefined>}
        onSubmit={onSubmit}
        allowEndpointEdit
        {...props}
      />
    );
  }
);

const EditCustomModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<OpenAIModel>({
      resolver: zodResolver(editOpenAIModelFormSchema),
      defaultValues: model as OpenAIModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOpenAIModelForm
        form={
          form as UseFormReturn<NewOpenAIModel | OpenAIModel, any, undefined>
        }
        onSubmit={onSubmit as (model: NewModel | Model) => void}
        allowEndpointEdit
        {...props}
      />
    );
  }
);

export default {
  Azure: {
    New: NewAzureModelForm,
    Edit: EditAzureModelForm,
  },
  OpenAI: {
    New: NewOpenAIModelForm,
    Edit: EditOpenAIModelForm,
  },
  Claude: {
    New: NewClaudeModelForm,
    Edit: EditClaudeModelForm,
  },
  Ollama: {
    New: NewOllamaModelForm,
    Edit: EditOllamaModelForm,
  },
  Openrouter: {
    New: NewOpenrouterModelForm,
    Edit: EditOpenAIModelForm, // use EditOpenAIModelForm for editing
  },
  Deepseek: {
    New: NewDeepseekModelForm,
    Edit: EditOpenAIModelForm, // use EditOpenAIModelForm for editing
  },
  Xai: {
    New: NewXaiModelForm,
    Edit: EditOpenAIModelForm, // use EditOpenAIModelForm for editing
  },
  Google: {
    New: NewGoogleModelForm,
    Edit: EditGoogleModelForm,
  },
  CUSTOM: {
    New: NewCustomModelForm,
    Edit: EditCustomModelForm,
  },
};
